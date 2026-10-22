import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const unit = (file) => JSON.parse(readFileSync(join(root, file), "utf8")).scripts.test;
const forbidden = [
  "test:samples",
  "test:e2e",
  "--workspaces",
  "Datasources.slnx",
  "--workspace",
  "npm run build",
  "datasources-mysql",
  "datasources-postgres",
  "datasources-json",
  "datasources-xml",
  "datasources-yaml",
  "datasources-mongodb",
  "datasources-dynamodb",
  "datasources-oracle",
  "datasources-sqlserver",
  "Datasources.Mysql",
  "Datasources.Postgres",
  "Datasources.Json",
  "Datasources.Xml",
  "Datasources.Yaml",
  "Datasources.MongoDb",
  "Datasources.DynamoDb",
  "Datasources.Oracle",
  "Datasources.SqlServer",
];

for (const [file, script] of [
  ["package.json", unit("package.json")],
  ["typescript/package.json", unit("typescript/package.json")],
])
  test(`${file} unit test stays on sqlite and memory`, () => {
    for (const token of forbidden)
      assert.ok(!script.includes(token), `${file} scripts.test must not include ${token}`);
  });
