// Workspace installs hoist every provider. Core tests only declare json and memory,
// so the other packages are shadowed here and DriverLoader reports them missing.
import { registerHooks } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const blocked = new Set([
  "@deterministic-code/datasources-sqlite",
  "@deterministic-code/datasources-xml",
  "@deterministic-code/datasources-yaml",
  "@deterministic-code/datasources-mysql",
  "@deterministic-code/datasources-sqlserver",
  "@deterministic-code/datasources-postgres",
  "@deterministic-code/datasources-oracle",
  "@deterministic-code/datasources-mongodb",
  "@deterministic-code/datasources-dynamodb",
]);

const stub = pathToFileURL(join(dirname(fileURLToPath(import.meta.url)), "test-missing-driver.cjs")).href;

registerHooks({
  resolve: (specifier, context, nextResolve) =>
    blocked.has(specifier) ? { url: stub, shortCircuit: true } : nextResolve(specifier, context),
});
