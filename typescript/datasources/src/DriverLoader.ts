import { readFileSync } from "node:fs";
import { createRequire, registerHooks } from "node:module";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import type { IDriver } from "./IDriver.ts";
import type { ProviderCatalog } from "./ProviderCatalog.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";

type DriverCtor = new () => IDriver;
type DriverModule = Record<string, DriverCtor>;
type TypeScript = {
  transpileModule: (
    source: string,
    options: {
      compilerOptions: { module: number; target: number };
      fileName: string;
    },
  ) => { outputText: string };
  ModuleKind: { ESNext: number };
  ScriptTarget: { ES2022: number };
};

const require = createRequire(import.meta.url);
const requirers = () => [
  createRequire(join(process.cwd(), "package.json")),
  ...(process.argv[1] ? [createRequire(process.argv[1])] : []),
  require,
];

const missingModule = (error: unknown): boolean =>
  error instanceof Error &&
  "code" in error &&
  (error.code === "MODULE_NOT_FOUND" ||
    error.code === "ERR_MODULE_NOT_FOUND" ||
    error.code === "ERR_PACKAGE_PATH_NOT_EXPORTED");
const virtual = new Map<string, string>();
const virtualBySource = new Map<string, string>();
let typeScriptHook = false;
let compiling = false;
let compilingRoot = "";
let nextVirtual = 0;

/* v8 ignore start */
const underRoot = (url: string): boolean => {
  if (!url.startsWith("file:")) return false;
  const path = fileURLToPath(url);
  return path === compilingRoot || path.startsWith(`${compilingRoot}/`);
};
/* v8 ignore stop */

const installTypeScriptHook = (): void => {
  if (typeScriptHook) return;
  const typescript = require("typescript") as TypeScript;
  /* v8 ignore start */
  registerHooks({
    resolve: (specifier, context, nextResolve) => {
      const parentURL = context.parentURL ? virtual.get(context.parentURL) ?? context.parentURL : undefined;
      const resolved = nextResolve(specifier, parentURL ? { ...context, parentURL } : context);
      /* v8 ignore next */
      if (!compiling || !resolved.url.endsWith(".ts") || !underRoot(resolved.url)) return resolved;
      const existing = virtualBySource.get(resolved.url);
      /* v8 ignore next */
      if (existing) return { url: existing, shortCircuit: true };
      const url = pathToFileURL(`${tmpdir()}/datasources-plugin-${nextVirtual++}.mjs`).href;
      virtual.set(url, resolved.url);
      virtualBySource.set(resolved.url, url);
      return { url, shortCircuit: true };
    },
    load: (url, context, nextLoad) => {
      const original = virtual.get(url) ?? (compiling && url.endsWith(".ts") && underRoot(url) ? url : undefined);
      /* v8 ignore next */
      if (!original) return nextLoad(url, context);
      const filename = fileURLToPath(original);
      const source = typescript.transpileModule(readFileSync(filename, "utf8"), {
        compilerOptions: {
          module: typescript.ModuleKind.ESNext,
          target: typescript.ScriptTarget.ES2022,
        },
        fileName: filename,
      }).outputText;
      return { format: "module", source, shortCircuit: true };
    },
  });
  /* v8 ignore stop */
  typeScriptHook = true;
};

const instantiate = (catalog: ProviderCatalog): IDriver => {
  compiling = true;
  try {
    return instantiateLoaded(catalog);
  } finally {
    compiling = false;
    compilingRoot = "";
  }
};

const instantiateLoaded = (catalog: ProviderCatalog): IDriver => {
  let last: unknown;
  for (const req of requirers()) {
    try {
      compilingRoot = dirname(req.resolve(catalog.location));
      const loaded = req(catalog.location) as DriverModule;
      const Type = loaded[catalog.module];
      if (typeof Type !== "function") throw new ProviderNotInstalledError(catalog.name);
      return new Type();
    } catch (error) {
      last = error;
      if (error instanceof ProviderNotInstalledError || !missingModule(error)) throw error;
    }
  }
  throw last instanceof ProviderNotInstalledError ? last : new ProviderNotInstalledError(catalog.name);
};

export class DriverLoader {
  static load = (catalog: ProviderCatalog): IDriver => {
    try {
      return instantiate(catalog);
    } catch (error) {
      if (error instanceof ProviderNotInstalledError) throw error;
      try {
        installTypeScriptHook();
        return instantiate(catalog);
      } catch {
        throw new ProviderNotInstalledError(catalog.name);
      }
    }
  };
}
