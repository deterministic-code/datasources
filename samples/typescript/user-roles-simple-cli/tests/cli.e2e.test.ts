import { execFileSync } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createConnection } from "mysql2/promise";
import pg from "pg";
import { describe, expect, it } from "vitest";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const cli = (url: string, schema: string, ...args: string[]) =>
  execFileSync(process.execPath, ["--import", "tsx", "src/main.ts", ...args], {
    cwd: sample,
    encoding: "utf8",
    env: { ...process.env, DATASOURCE_URL: url, DATABASE_SCHEMA: schema },
  });

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

const script = ({ url, schema }: { url: string; schema: string }) => {
  cli(url, schema, "add-role", "admin");
  cli(url, schema, "add-group", "staff");
  cli(url, schema, "add-user", "alice", "secret");
  cli(url, schema, "add-user-role", "alice", "admin");
  cli(url, schema, "add-user-group", "alice", "staff");
  const listed = JSON.parse(cli(url, schema, "list-users-roles-groups")) as {
    username: string;
    is_enabled: boolean;
    roles: { name: string }[];
    groups: { name: string }[];
  }[];
  expect(listed[0]?.username).toBe("alice");
  expect(listed[0]?.is_enabled).toBe(true);
  expect(listed[0]?.roles[0]?.name).toBe("admin");
  expect(listed[0]?.groups[0]?.name).toBe("staff");
  const raw = cli(url, schema, "list-users-roles-groups");
  expect(raw).not.toContain("password");
  expect(raw).not.toContain("ugly_name");
  cli(url, schema, "disable-user", "alice");
  expect(JSON.parse(cli(url, schema, "list-users-roles-groups"))[0].is_enabled).toBe(false);
};

describe("cli e2e", () => {
  it("sqlite", () => {
    script(setup(`sqlite:///${join(mkdtempSync(join(tmpdir(), "ur-")), "app.db")}`));
  });

  it("mysql", async () => {
    const url = process.env.MYSQL_URL;
    if (!url) return;
    const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = await createConnection(url);
    await admin.query(`CREATE DATABASE \`${name}\``);
    try {
      const isolated = new URL(url);
      isolated.pathname = `/${name}`;
      script(setup(isolated.toString()));
    } finally {
      await admin.query(`DROP DATABASE \`${name}\``);
      await admin.end();
    }
  });

  it("postgres", async () => {
    const url = process.env.POSTGRES_URL;
    if (!url) return;
    const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
    const admin = new pg.Client({ connectionString: url });
    await admin.connect();
    await admin.query(`CREATE SCHEMA ${schema}`);
    try {
      script(setup(url, schema));
    } finally {
      await admin.query(`DROP SCHEMA ${schema} CASCADE`);
      await admin.end();
    }
  });
});
