import { spawnSync } from "node:child_process";
import { cp, mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const packages = join(root, "node_modules/@deterministic-code");
const require = createRequire(import.meta.url);

const esbuildReady = () => {
  try {
    require("esbuild");
    return true;
  } catch {
    return false;
  }
};

if (!esbuildReady()) {
  const install = spawnSync(process.execPath, [require.resolve("esbuild/install.js")], { stdio: "inherit" });
  if (install.status !== 0) process.exit(install.status ?? 1);
}

const { build } = await import("esbuild");

const rewriteTs = {
  name: "rewrite-ts",
  setup(result) {
    result.onLoad({ filter: /\.ts$/ }, async (args) => ({
      contents: (await readFile(args.path, "utf8")).replace(/(["'])(\.[^"']+?)\.ts\1/g, "$1$2.js$1"),
      loader: "ts",
    }));
  },
};

const walk = async (dir, skip) => {
  const files = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "dist" || entry.name === "node_modules" || entry.name === "__tests__") continue;
      files.push(...(await walk(path, skip)));
    } else if (entry.name.endsWith(".ts") && !entry.name.endsWith(".d.ts") && !skip(path)) {
      files.push(path);
    }
  }
  return files;
};

const skipPack = (path) =>
  path.endsWith(".test.ts") || path.endsWith("generate-datasource-type-validators.ts") || path.includes(`${join("src", "templates")}`);

const transpile = async (name) => {
  const src = join(packages, name, "src");
  const dist = join(packages, name, "dist");
  await build({
    entryPoints: await walk(src, skipPack),
    outdir: dist,
    outbase: src,
    format: "esm",
    platform: "node",
    packages: "external",
    plugins: [rewriteTs],
    logLevel: "warning",
  });
  await cp(join(src, "templates"), join(dist, "templates"), { recursive: true }).catch(() => undefined);
};

const parserSource = join(packages, "deterministic-specifications-typescript/parser/parser/specification-parser.ts");
const parserText = await readFile(parserSource, "utf8");
if (!parserText.startsWith("// @ts-nocheck")) await writeFile(parserSource, `// @ts-nocheck\n${parserText}`);

await build({
  entryPoints: [join(packages, "deterministic-specifications-typescript/parser/specification-parser.ts")],
  bundle: true,
  format: "esm",
  platform: "node",
  packages: "external",
  outfile: join(packages, "deterministic-specifications-typescript/parser/dist/specification-parser.js"),
  logLevel: "warning",
});

for (const name of ["generators-common", "typescript-generators", "csharp-generators", "rust-generators", "generators-sql"]) {
  await transpile(name);
}

const tsc = spawnSync(
  process.execPath,
  [join(root, "node_modules/typescript/bin/tsc"), "-p", join(packages, "generators-common/tsconfig.build.json"), "--noCheck", "--declaration", "--emitDeclarationOnly"],
  { stdio: "inherit" },
);
if (tsc.status !== 0) process.exit(tsc.status ?? 1);

const stub = `import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
export declare const generate: (ctx: GenerateContext) => Promise<GenerateEntry[]>;
`;
const stubs = [
  ["typescript-generators", "generate-datasource-types"],
  ["typescript-generators", "generate-view-types"],
  ["csharp-generators", "generate-datasource-types"],
  ["csharp-generators", "generate-view-types"],
  ["rust-generators", "generate-datasource-types"],
  ["rust-generators", "generate-view-types"],
  ["generators-sql", "generate-sql"],
];
for (const [name, file] of stubs) {
  const path = join(packages, name, "dist", `${file}.d.ts`);
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, stub);
}
