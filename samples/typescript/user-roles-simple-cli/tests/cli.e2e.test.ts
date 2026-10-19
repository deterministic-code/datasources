import { ExpectConsoleMiddleware, type IConnection } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { execFileSync, spawnSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createConnection } from "mysql2/promise";
import pg from "pg";
import { describe, it } from "vitest";
import { parse as parseYaml } from "yaml";
import { App } from "../src/app.ts";
import { normalizeConsoleLine } from "../src/console-log.ts";
import { migrate } from "../src/migrate.ts";
import { parse } from "../src/parse.ts";
import { freshCompose, stopCompose } from "../../../../typescript/ensure-compose.ts";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
type Expect = {
  exit: number;
  stdout: string;
  stderr: string;
  queries?: string[];
  inserts?: string[];
  calls?: string[];
};

const spec = parseYaml(readFileSync(resolve(sample, "../../user-roles-simple-cli-shared/cli.yaml"), "utf8")) as {
  now: string;
  steps: { argv: string[]; expect: Expect }[];
};

const signature = (line: string): string => {
  const split = line.indexOf("]-");
  const body = split >= 0 ? line.slice(split + 2) : line;
  const duration = body.lastIndexOf("] ");
  return duration >= 0 && (body.endsWith("ms") || body.endsWith("μs")) ? body.slice(0, duration + 1) : body;
};

const startSql = (line: string): string | undefined => {
  const signed = signature(line);
  const prefix = "[datasource][Start]-[";
  return signed.startsWith(prefix) && signed.endsWith("]") ? signed.slice(prefix.length, -1) : undefined;
};

const unquote = (text: string): string => text.replace(/`([^`]+)`/g, "$1").replace(/"([^"]+)"/g, "$1");

const commands = (stderr: string): { leftover: string; sql: string[] } => {
  const leftover: string[] = [];
  const sql: string[] = [];
  for (const line of stderr.replace(/\r\n/g, "\n").split("\n")) {
    if (line.includes("[datasource][Start]") || line.includes("[datasource][Finish]")) {
      const text = startSql(line);
      if (text) sql.push(text);
    } else if (line.length > 0) leftover.push(line);
  }
  return { leftover: leftover.join("\n"), sql };
};

const withSchema = (url: string, schema: string) =>
  schema ? `${url}${url.includes("?") ? "&" : "?"}schema=${schema}` : url;

const cli = (url: string, schema: string, provider: string, args: string[]) => {
  const result = spawnSync(process.execPath, ["--import", "tsx", "src/main.ts", ...args], {
    cwd: sample,
    encoding: "utf8",
    env: { ...process.env, PROVIDER: provider, DATASOURCE_URL: withSchema(url, schema), CLI_NOW: spec.now },
  });
  return { exit: result.status ?? 1, stdout: result.stdout ?? "", stderr: result.stderr ?? "" };
};

const setup = (url: string, schema = "", provider = "sqlite") => {
  writeFileSync(
    join(sample, ".env"),
    `INSTALLED_PROVIDERS=sqlite,mysql,postgres\nPROVIDER=${provider}\nDATASOURCE_URL=${withSchema(url, schema)}\n`,
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
    const got = cli(url, schema, provider, step.argv);
    const cmd = step.argv.join(" ");
    const { leftover, sql } = commands(got.stderr);
    const expectedSql = [...(step.expect.queries ?? []), ...(provider === "mysql" ? (step.expect.calls ?? []) : (step.expect.inserts ?? []))];
    const actualSql = sql.map((text) => (text.toUpperCase().startsWith("SELECT") ? unquote(text) : text));
    if (verbose()) {
      console.log(`${provider}: ${cmd}`);
      console.log(`  assert exit ${got.exit} == ${step.expect.exit}`);
      console.log(`  assert stdout [${got.stdout}] == [${step.expect.stdout}]`);
      console.log(`  assert stderr [${leftover}] == [${step.expect.stderr}]`);
      console.log(`  assert commands [${actualSql.join(" | ")}] == [${expectedSql.join(" | ")}]`);
    }
    if (
      got.exit !== step.expect.exit ||
      got.stdout !== step.expect.stdout ||
      leftover !== step.expect.stderr ||
      actualSql.join("\0") !== expectedSql.join("\0")
    )
      throw new Error(
        `${provider}: ${cmd}: first mismatch\nexpected exit ${step.expect.exit} stdout [${step.expect.stdout}] stderr [${step.expect.stderr}] commands [${expectedSql.join(" | ")}]\nactual exit ${got.exit} stdout [${got.stdout}] stderr [${leftover}] commands [${actualSql.join(" | ")}]`,
      );
  }
};

describe.sequential("cli e2e", () => {
  it("sqlite", () => {
    const dir = mkdtempSync(join(tmpdir(), "ur-"));
    try {
      script(setup(`sqlite:///${join(dir, "app.db")}`, "", "sqlite"), "sqlite");
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  }, 180_000);

  it("mysql", async () => {
    freshCompose("mysql");
    const url = process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:3306/datasources";
    const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = await createConnection(url);
    await admin.query(`CREATE DATABASE \`${name}\``);
    try {
      const isolated = new URL(url);
      isolated.pathname = `/${name}`;
      script(setup(isolated.toString(), "", "mysql"), "mysql");
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
      script(setup(url, schema, "postgres"), "postgres");
    } finally {
      await admin.query(`DROP SCHEMA ${schema} CASCADE`);
      await admin.end();
      stopCompose("postgres");
    }
  }, 180_000);

  const transcript = async (file: string, connection: IConnection, procedures: boolean) => {
    const expected = readFileSync(resolve(sample, "../../user-roles-simple-cli-shared", file), "utf8")
      .split("\n")
      .filter((line) => line.length > 0)
      .map(normalizeConsoleLine);
    await connection.open();
    try {
      const command = connection.createCommand();
      await migrate(command);
      const now = () => new Date(spec.now);
      const middleware = new ExpectConsoleMiddleware(expected, now, normalizeConsoleLine);
      command.addMiddleware(middleware);
      const app = new App(command, now, () => "hash", () => new Uint8Array([1, 2, 3]), procedures);
      for (const step of spec.steps) await app.run(parse(step.argv));
      middleware.complete();
    } finally {
      await connection.close();
    }
  };

  it("mysql console transcript", async () => {
    freshCompose("mysql");
    const url = process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:3306/datasources";
    const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const procedures = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = await createConnection(url);
    await admin.query(`CREATE DATABASE \`${name}\``);
    await admin.query(`CREATE DATABASE \`${procedures}\``);
    try {
      const isolated = new URL(url);
      isolated.pathname = `/${name}`;
      await transcript("mysql.log", new MysqlDriver().connection({ DATASOURCE_URL: isolated.toString() }), false);
      const procedureUrl = new URL(url);
      procedureUrl.pathname = `/${procedures}`;
      await transcript("mysql-procedures.log", new MysqlDriver().connection({ DATASOURCE_URL: procedureUrl.toString() }), true);
    } finally {
      await admin.query(`DROP DATABASE \`${name}\``);
      await admin.query(`DROP DATABASE \`${procedures}\``);
      await admin.end();
      stopCompose("mysql");
    }
  }, 180_000);

  it("postgres console transcript", async () => {
    freshCompose("postgres");
    const url = process.env.POSTGRES_URL || "postgres://ds:ds@127.0.0.1:5432/datasources";
    const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = new pg.Client({ connectionString: url });
    await admin.connect();
    await admin.query(`CREATE SCHEMA ${schema}`);
    try {
      await transcript("postgres.log", new PostgresDriver().connection({ DATASOURCE_URL: withSchema(url, schema) }), false);
    } finally {
      await admin.query(`DROP SCHEMA ${schema} CASCADE`);
      await admin.end();
      stopCompose("postgres");
    }
  }, 180_000);
});
