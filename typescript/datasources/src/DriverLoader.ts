import { readFileSync } from "node:fs";
import { createRequire, registerHooks } from "node:module";
import { tmpdir } from "node:os";
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
const virtual = new Map<string, string>();
const virtualBySource = new Map<string, string>();
let typeScriptHook = false;
let compiling = false;
let nextVirtual = 0;

const installTypeScriptHook = (): void => {
  if (typeScriptHook) return;
  const typescript = require("typescript") as TypeScript;
  /* v8 ignore start */
  registerHooks({
    resolve: (specifier, context, nextResolve) => {
      const parentURL = context.parentURL ? virtual.get(context.parentURL) ?? context.parentURL : undefined;
      const resolved = nextResolve(specifier, parentURL ? { ...context, parentURL } : context);
      /* v8 ignore next */
      if (!compiling || !resolved.url.endsWith(".ts")) return resolved;
      const existing = virtualBySource.get(resolved.url);
      /* v8 ignore next */
      if (existing) return { url: existing, shortCircuit: true };
      const url = pathToFileURL(`${tmpdir()}/datasources-plugin-${nextVirtual++}.mjs`).href;
      virtual.set(url, resolved.url);
      virtualBySource.set(resolved.url, url);
      return { url, shortCircuit: true };
    },
    load: (url, context, nextLoad) => {
      const original = virtual.get(url) ?? (compiling && url.endsWith(".ts") ? url : undefined);
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
  }
};

const instantiateLoaded = (catalog: ProviderCatalog): IDriver => {
  const loaded = require(catalog.location) as DriverModule;
  const Type = loaded[catalog.module];
  if (typeof Type !== "function") throw new ProviderNotInstalledError(catalog.name);
  return new Type();
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
