import { execFileSync } from "node:child_process";
import { mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Database from "better-sqlite3";
import { createConnection } from "mysql2/promise";
import pg from "pg";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import { VersionSnapshot } from "@deterministic-code/deterministic-generators-common/parser";
const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const spec = resolve(sample, "../../user-roles-simple-cli-shared/deterministic");

const file = (provider: string, connection: string) =>
  provider === "sqlite" && connection.startsWith("sqlite:///") ? connection.slice("sqlite:///".length) : connection;

const migrate = (command: string, provider: string, connection: string, root: string, extra: string[] = []) => {
  const path = file(provider, connection);
  if (command === "migrate-up") {
    execFileSync("npx", ["migrate-init", "--language", "typescript", "--dialect", provider], { encoding: "utf8", cwd: sample });
    execFileSync("npx", ["migrate-setup", "--provider", provider, "--connection", path], { encoding: "utf8", cwd: sample });
  }
  execFileSync(
    "npx",
    [command, "--provider", provider, "--connection", path, "--migrations-root", root, ...extra],
    { encoding: "utf8", cwd: sample },
  );
};

const providers = (): string[] => [
  "sqlite",
  ...(process.env.MYSQL_URL ? ["mysql"] : []),
  ...(process.env.POSTGRES_URL ? ["postgres"] : []),
];

const isolate = async (provider: string, dir: string) => {
  if (provider === "sqlite") return { url: `sqlite:///${join(dir, "app.sqlite")}`, drop: async () => undefined };
  if (provider === "mysql") {
    const base = process.env.MYSQL_URL!;
    const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = await createConnection(base);
    await admin.query(`CREATE DATABASE \`${name}\``);
    const url = new URL(base);
    url.pathname = `/${name}`;
    return { url: url.toString(), drop: async () => { await admin.query(`DROP DATABASE IF EXISTS \`${name}\``); await admin.end(); } };
  }
  const base = process.env.POSTGRES_URL!;
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = new pg.Client({ connectionString: base });
  await admin.connect();
  await admin.query(`CREATE SCHEMA ${name}`);
  return {
    url: `${base}${base.includes("?") ? "&" : "?"}schema=${name}`,
    drop: async () => { await admin.query(`DROP SCHEMA IF EXISTS ${name} CASCADE`); await admin.end(); },
  };
};

const catalog = async (provider: string, url: string) => {
  if (provider === "sqlite") {
    const db = new Database(new URL(url).pathname);
    const tables = (db.prepare("SELECT name FROM sqlite_master WHERE type = 'table'").all() as { name: string }[]).map((row) => row.name);
    const columns = (table: string) => (db.prepare(`PRAGMA table_info("${table}")`).all() as { name: string }[]).map((row) => row.name);
    return { tables, columns, close: () => db.close() };
  }
  if (provider === "mysql") {
    const connection = await createConnection(url);
    const [tables] = (await connection.query("SELECT table_name AS name FROM information_schema.tables WHERE table_schema = DATABASE()")) as [{ name: string }[]];
    const columns = async (table: string) => {
      const [rows] = (await connection.query("SELECT column_name AS name FROM information_schema.columns WHERE table_schema = DATABASE() AND table_name = ?", [table])) as [{ name: string }[]];
      return rows.map((row) => row.name);
    };
    return { tables: tables.map((row) => row.name), columns, close: () => connection.end() };
  }
  const client = new pg.Client({ connectionString: url });
  await client.connect();
  const tables = (await client.query<{ name: string }>("SELECT table_name AS name FROM information_schema.tables WHERE table_schema = current_schema()")).rows.map((row) => row.name);
  const columns = async (table: string) =>
    (await client.query<{ name: string }>("SELECT column_name AS name FROM information_schema.columns WHERE table_schema = current_schema() AND table_name = $1", [table])).rows.map((row) => row.name);
  return { tables, columns, close: () => client.end() };
};

describe("versioned migrations", () => {
  it("replays frozen diffs to live types.yaml", async () => {
    const snapshot = await VersionSnapshot.read(spec);
    const live = VersionSnapshot.combine([
      { "types.yaml": { added: true, content: parse(readFileSync(join(spec, "types.yaml"), "utf8")) } },
    ]);
    expect(snapshot.yaml("types.yaml")).toBe(live.yaml("types.yaml"));
  });

  it.each(providers())("creates the live schema from the initial version (%s)", async (provider) => {
    const out = mkdtempSync(join(tmpdir(), `user-roles-mig-${provider}-`));
    const isolated = await isolate(provider, out);
    try {
      const source = join(sample, "sql", provider, "migrations");
      expect(readdirSync(source).filter((name) => /^\d{4}_up\.sql$/.test(name)).sort()).toEqual(["0001_up.sql"]);
      migrate("migrate-up", provider, isolated.url, join(sample, "sql"));
      const applied = await catalog(provider, isolated.url);
      expect(applied.tables).toEqual(expect.arrayContaining(["users", "roles", "groups", "user_groups", "user_roles"]));
      expect(applied.tables).not.toEqual(expect.arrayContaining(["sessions", "audit_logs"]));
      for (const table of ["users", "roles", "groups", "user_groups", "user_roles"]) {
        expect(await applied.columns(table)).toContain("uuid");
      }
      expect(await applied.columns("roles")).toContain("description");
      expect(await applied.columns("groups")).toContain("description");
      await applied.close();
      migrate("migrate-down", provider, isolated.url, join(sample, "sql"), ["--confirm"]);
      const rolled = await catalog(provider, isolated.url);
      expect(rolled.tables).not.toEqual(expect.arrayContaining(["users", "roles", "groups"]));
      await rolled.close();
    } finally {
      await isolated.drop();
      rmSync(out, { recursive: true, force: true });
    }
  }, 180_000);
});
