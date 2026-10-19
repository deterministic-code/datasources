// Workspace installs hoist every provider. Core tests only declare json and memory,
// so the other packages are shadowed here and DriverLoader reports them missing.
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "node_modules/@datasources");
for (const name of [
  "sqlite",
  "xml",
  "yaml",
  "mysql",
  "sqlserver",
  "postgres",
  "oracle",
  "mongodb",
  "dynamodb",
]) {
  const dir = join(root, name);
  mkdirSync(dir, { recursive: true });
  writeFileSync(
    join(dir, "package.json"),
    JSON.stringify({ name: `@deterministic-code/datasources-${name}`, type: "module" }),
  );
  writeFileSync(join(dir, "index.js"), "throw new Error(\"not installed\");\n");
}
