import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Database from "better-sqlite3";
import { createConnection } from "mysql2/promise";
import pg from "pg";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import {
  applyDocument,
  snapshotsEqual,
  type Json,
} from "../../../../typescript/code-generators/src/sql/common/version-apply.ts";
import { migrateDown, migrateUp } from "../../../../scripts/migraters.mjs";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = resolve(sample, "../../user-roles-simple-cli-shared/deterministic");

const create = (root: string) =>
  execFileSync(
    process.execPath,
    ["--import", "tsx", join(repo, "cli/src/main.ts"), "migrations", "create", "--input", spec, "--migrations-root", join(root, "sql")],
    { encoding: "utf8", cwd: join(repo, "cli") },
  );

const stage = (from: string, to: string, n: number) => {
  mkdirSync(to, { recursive: true });
  for (const i of [1, 2, 3].filter((step) => step <= n)) {
    const name = String(i).padStart(4, "0");
    for (const side of ["up", "down"]) copyFileSync(join(from, `${name}_${side}.sql`), join(to, `${name}_${side}.sql`));
  }
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
  it("replays frozen diffs to live types.yaml", () => {
    let snapshot: { [path: string]: Json } = {};
    for (const name of readdirSync(spec).filter((name) => /^version-\d{8}T\d{6}Z-\d{3}$/.test(name)).sort()) {
      snapshot = applyDocument(snapshot, parse(readFileSync(join(spec, name, "diff.yaml"), "utf8")) as { [path: string]: Json });
    }
    expect(snapshotsEqual(snapshot["types.yaml"], parse(readFileSync(join(spec, "types.yaml"), "utf8")) as Json)).toBe(true);
  });

  it.each(providers())("drops extra fields and tables across three versions (%s)", async (provider) => {
    const out = mkdtempSync(join(tmpdir(), `user-roles-mig-${provider}-`));
    const isolated = await isolate(provider, out);
    try {
      create(out);
      const source = join(out, "sql", provider, "migrations");
      expect(readdirSync(source).filter((name) => /^\d{4}_up\.sql$/.test(name)).sort()).toEqual(["0001_up.sql", "0002_up.sql", "0003_up.sql"]);
      const staged = join(out, "staged");
      const apply = (n: number) => {
        stage(source, join(staged, provider, "migrations"), n);
        migrateUp({ provider, connection: isolated.url, migrationsRoot: staged });
      };
      apply(1);
      const first = await catalog(provider, isolated.url);
      expect(first.tables).toEqual(expect.arrayContaining(["users", "roles", "groups", "sessions", "audit_logs"]));
      expect(await first.columns("users")).toContain("nickname");
      expect(await first.columns("roles")).toContain("description");
      await first.close();
      apply(2);
      const second = await catalog(provider, isolated.url);
      expect(second.tables).not.toContain("sessions");
      expect(await second.columns("users")).not.toContain("nickname");
      expect(await second.columns("roles")).toContain("description");
      await second.close();
      apply(3);
      const third = await catalog(provider, isolated.url);
      expect(third.tables).not.toContain("audit_logs");
      expect(await third.columns("roles")).not.toContain("description");
      expect(third.tables).toEqual(expect.arrayContaining(["users", "roles", "groups"]));
      await third.close();
      migrateDown({ provider, connection: isolated.url, migrationsRoot: staged });
      const rolled = await catalog(provider, isolated.url);
      expect(rolled.tables).not.toContain("sessions");
      expect(await rolled.columns("users")).not.toContain("nickname");
      expect(await rolled.columns("roles")).toContain("description");
      await rolled.close();
    } finally {
      await isolated.drop();
      rmSync(out, { recursive: true, force: true });
    }
  }, 180_000);
});
