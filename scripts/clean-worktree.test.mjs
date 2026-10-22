import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { clean } from "./clean-worktree.mjs";

const file = (root, path, body = "x") => {
  const full = join(root, path);
  mkdirSync(join(full, ".."), { recursive: true });
  writeFileSync(full, body);
  return full;
};

test("clean removes disposable outputs and keeps tracked and skipped paths", () => {
  const root = mkdtempSync(join(tmpdir(), "clean-worktree-"));
  const gone = [
    "scripts/artifacts/e2e-matrix/index.html",
    "csharp/artifacts/Datasources.1.0.0.nupkg",
    "docs/evals/workspace/iteration-1/id/with_skill/a.ts",
    "docs/.vitepress/cache/x",
    "docs/.vitepress/dist/index.html",
    ".deterministic-cache/hit",
    "coverage/lcov.info",
    "TestResults/out.trx",
    "lcov.info",
    "coverage.cobertura.xml",
    "coverage.info",
    "typescript/datasources/tsconfig.tsbuildinfo",
    "local.nupkg",
    "samples/csharp/user-roles-simple-cli/app.sqlite",
    "samples/typescript/user-roles-simple-cli/app.db",
    "samples/typescript/user-roles-simple-cli/app.db-wal",
    "samples/typescript/user-roles-simple-cli/app.sqlite-shm",
    "samples/typescript/user-roles-simple-cli/app.sqlite-journal",
    "samples/typescript/user-roles-simple-cli/var/folders/qz/tmp/app.db",
  ];
  for (const path of gone) file(root, path);
  file(root, "cli/bin/datasources.mjs");
  file(root, "node_modules/keep/pkg.json");
  file(root, "target/debug/lib.rlib");
  file(root, "bin/Debug/out.dll");
  file(root, "obj/Debug/out.dll");
  file(root, "keep/app.sqlite");
  file(root, ".env", "SECRET=1");
  file(root, "samples/user-roles-simple-cli-shared/sqlite.log");

  clean(root, new Set(["keep/app.sqlite"]));

  for (const path of gone) assert.equal(existsSync(join(root, path)), false, path);
  assert.equal(existsSync(join(root, "scripts/artifacts")), false);
  assert.equal(existsSync(join(root, "samples/typescript/user-roles-simple-cli/var")), false);
  assert.ok(existsSync(join(root, "cli/bin/datasources.mjs")));
  assert.ok(existsSync(join(root, "node_modules/keep/pkg.json")));
  assert.ok(existsSync(join(root, "target/debug/lib.rlib")));
  assert.ok(existsSync(join(root, "bin/Debug/out.dll")));
  assert.ok(existsSync(join(root, "obj/Debug/out.dll")));
  assert.ok(existsSync(join(root, "keep/app.sqlite")));
  assert.ok(existsSync(join(root, ".env")));
  assert.ok(existsSync(join(root, "samples/user-roles-simple-cli-shared/sqlite.log")));
});
