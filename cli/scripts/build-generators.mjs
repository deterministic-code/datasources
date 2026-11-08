import { spawnSync } from "node:child_process";
import { cp, glob, mkdir, readdir, readFile, writeFile } from "node:fs/promises";
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

const typeEmitters = [
  ["csharp-generators", "src/generate-datasource-types.ts"],
  ["csharp-generators", "src/generate-view-types.ts"],
  ["typescript-generators", "src/emit-shaped-types.ts"],
  ["rust-generators", "src/generate-datasource-types.ts"],
  ["rust-generators", "src/generate-view-types.ts"],
];

const unwrapInferredSpec = async () => {
  for (const [name, file] of typeEmitters) {
    const path = join(packages, name, file);
    let text = await readFile(path, "utf8");
    text = text.replace(/^import \{ inferredSpec \} from "\.\/inferred-spec\.ts";\n/m, "");
    text = text.replace(
      /inferredSpec\(\s*await DeterministicParser\(ctx\.reader\)\.parse\(ctx\.settings\),?\s*\)/g,
      "await DeterministicParser(ctx.reader).parse(ctx.settings)",
    );
    await writeFile(path, text);
  }
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

const copyReferencedFieldTypes = `const referencedField = (
  types: Type[],
  references: TypeField["references"],
): TypeField | undefined => {
  if (references === undefined) return undefined;
  const [table, column] = Array.isArray(references)
    ? references
    : references.split(".");
  if (table === undefined || column === undefined) return undefined;
  return types.find((type) => type.name === table)?.fields.find((field) => field.name === column);
};

const needsReferencedType = (field: TypeField): boolean =>
  !field.isArray &&
  field.isMap !== true &&
  field.references !== undefined &&
  field.type === "string" &&
  field.size === undefined;

const referencedTypeReady = (field: TypeField): boolean =>
  field.type !== "string" || field.size !== undefined;

const copyReferencedFieldTypes = (types: Type[]): Type[] => {
  const working = types.map((type) => ({
    ...type,
    fields: type.fields.map((field) => ({ ...field })),
  }));
  for (let pass = 0; pass < working.length; pass++) {
    let changed = false;
    for (const type of working) {
      type.fields = type.fields.map((field) => {
        if (!needsReferencedType(field)) return field;
        const parent = referencedField(working, field.references);
        if (parent === undefined || !referencedTypeReady(parent)) return field;
        changed = true;
        return {
          ...field,
          type: parent.type,
          kind: parent.kind,
          base: parent.base,
          ...(parent.size !== undefined ? { size: parent.size } : {}),
        };
      });
    }
    if (!changed) break;
  }
  return working;
};

`;

const expandReturn = `  return types.map((type) => {
    const ids = type.ids ?? resolveIds(type.name, new Set());
    return {
      ...type,
      ...(ids !== undefined && ids.length > 0 ? { ids } : {}),
      fields: resolve(type.name, new Set()),
    };
  });
};`;

const copiedReturn = `  return copyReferencedFieldTypes(
    types.map((type) => {
      const ids = type.ids ?? resolveIds(type.name, new Set());
      return {
        ...type,
        ...(ids !== undefined && ids.length > 0 ? { ids } : {}),
        fields: resolve(type.name, new Set()),
      };
    }),
  );
};`;

const patchParser = async (parserRoot) => {
  const parserSource = join(parserRoot, "parser/specification-parser.ts");
  const parserText = await readFile(parserSource, "utf8");
  if (!parserText.startsWith("// @ts-nocheck")) await writeFile(parserSource, `// @ts-nocheck\n${parserText}`);
  const specificationSource = join(parserRoot, "specification.ts");
  let specification = await readFile(specificationSource, "utf8");
  if (!specification.includes("copyReferencedFieldTypes")) {
    if (!specification.includes("export const expandTypes")) throw new Error(`${specificationSource} has no expandTypes`);
    if (!specification.includes(expandReturn)) throw new Error(`${specificationSource} expandTypes return is not the parser expand`);
    specification = specification
      .replace("export const expandTypes", `${copyReferencedFieldTypes}export const expandTypes`)
      .replace(expandReturn, copiedReturn);
    await writeFile(specificationSource, specification);
  }
  if (!specification.includes("copyReferencedFieldTypes")) throw new Error(`${specificationSource} expandTypes must copy referenced field types`);
  await build({
    entryPoints: [join(parserRoot, "specification-parser.ts")],
    bundle: true,
    format: "esm",
    platform: "node",
    packages: "external",
    outfile: join(parserRoot, "dist/specification-parser.js"),
    logLevel: "warning",
  });
};

const repoRoot = join(root, "..");
for await (const spec of glob(
  "**/node_modules/@deterministic-code/deterministic-generators-common/typescript/parser/specification.ts",
  { cwd: repoRoot },
)) {
  await patchParser(dirname(join(repoRoot, spec)));
}

const includesRoot = join(packages, "deterministic-generators-common/typescript/includes");
await build({
  entryPoints: [join(includesRoot, "index.ts")],
  bundle: true,
  format: "esm",
  platform: "node",
  packages: "external",
  outfile: join(includesRoot, "dist/index.js"),
  logLevel: "warning",
});

await unwrapInferredSpec();

for (const name of ["generators-common", "typescript-generators", "csharp-generators", "rust-generators"]) {
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
];
for (const [name, file] of stubs) {
  const path = join(packages, name, "dist", `${file}.d.ts`);
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, stub);
}
