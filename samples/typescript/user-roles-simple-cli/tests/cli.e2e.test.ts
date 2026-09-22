import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createConnection } from "mysql2/promise";
import pg from "pg";
import { describe, it } from "vitest";
import { parse as parseYaml } from "yaml";
import { freshCompose, stopCompose } from "../../../../typescript/ensure-compose.ts";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = parseYaml(readFileSync(resolve(sample, "../../user-roles-simple-cli-shared/cli.yaml"), "utf8")) as {
  now: string;
  steps: { argv: string[]; expect: { exit: number; stdout: string; stderr: string } }[];
};

const cli = (url: string, schema: string, args: string[]) => {
  const result = spawnSync(process.execPath, ["--import", "tsx", "src/main.ts", ...args], {
    cwd: sample,
    encoding: "utf8",
    env: { ...process.env, DATASOURCE_URL: url, DATABASE_SCHEMA: schema, CLI_NOW: spec.now },
  });
  return { exit: result.status ?? 1, stdout: result.stdout ?? "", stderr: result.stderr ?? "" };
};

const setup = (url: string, schema = "") => {
  writeFileSync(
    join(sample, ".env"),
    `INSTALLED_PROVIDERS=sqlite,mysql,postgres\nDATASOURCE_URL=${url}\nDATABASE_SCHEMA=${schema}\n`,
  );
  execFileSync("npm", ["run", "setup", "--", "--cwd", sample, "--ecosystem", "ts", "--yes"], {
    cwd: repo,
    encoding: "utf8",
  });
  return { url, schema };
};

const verbose = () =>
  process.env.VERBOSE === "1" || process.env.VERBOSE === "true" || process.argv.includes("--verbose");

const script = ({ url, schema }: { url: string; schema: string }, provider: string) => {
  for (const step of spec.steps) {
    const got = cli(url, schema, step.argv);
    const cmd = step.argv.join(" ");
    if (verbose()) {
      console.log(`${provider}: ${cmd}`);
      console.log(`  assert exit ${got.exit} == ${step.expect.exit}`);
      console.log(`  assert stdout [${got.stdout}] == [${step.expect.stdout}]`);
      console.log(`  assert stderr [${got.stderr}] == [${step.expect.stderr}]`);
    }
    if (got.exit !== step.expect.exit || got.stdout !== step.expect.stdout || got.stderr !== step.expect.stderr)
      throw new Error(
        `${provider}: ${cmd}: first mismatch\nexpected exit ${step.expect.exit} stdout [${step.expect.stdout}] stderr [${step.expect.stderr}]\nactual exit ${got.exit} stdout [${got.stdout}] stderr [${got.stderr}]`,
      );
  }
};

describe.sequential("cli e2e", () => {
  it("sqlite", () => {
    const dir = mkdtempSync(join(tmpdir(), "ur-"));
    try {
      script(setup(`sqlite:///${join(dir, "app.db")}`), "sqlite");
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  it("mysql", async () => {
    freshCompose("mysql");
    const url = process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:3306/datasources";
    const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = await createConnection(url);
    await admin.query(`CREATE DATABASE \`${name}\``);
    try {
      const isolated = new URL(url);
      isolated.pathname = `/${name}`;
      script(setup(isolated.toString()), "mysql");
    } finally {
      await admin.query(`DROP DATABASE \`${name}\``);
      await admin.end();
      stopCompose("mysql");
    }
  }, 180_000);

  it("postgres", async () => {
    freshCompose("postgres");
    const url = process.env.POSTGRES_URL || "postgres://ds:ds@127.0.0.1:5432/datasources";
    const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = new pg.Client({ connectionString: url });
    await admin.connect();
    await admin.query(`CREATE SCHEMA ${schema}`);
    try {
      script(setup(url, schema), "postgres");
    } finally {
      await admin.query(`DROP SCHEMA ${schema} CASCADE`);
      await admin.end();
      stopCompose("postgres");
    }
  }, 180_000);
});
