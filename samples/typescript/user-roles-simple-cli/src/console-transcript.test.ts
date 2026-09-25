import { ExpectConsoleMiddleware } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { describe, expect, it } from "vitest";
import { App } from "./app.ts";
import { dialectTranscript, normalizeConsoleLine, sqlText } from "./console-log.ts";
import { migrate } from "./migrate.ts";
import { parse } from "./parse.ts";

const shared = resolve(dirname(fileURLToPath(import.meta.url)), "../../../user-roles-simple-cli-shared");

describe("console transcript", () => {
  it("normalizes dialect noise and matches sql text", () => {
    expect(normalizeConsoleLine("[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{password: hash, updated: AQID, blob: ab12, id: 11111111-1111-1111-1111-111111111111}] 2ms")).toBe(
      "[<date>]-[datasource][Request]-[{password: <token>, updated: <token>, blob: <token>, id: <uuid>}]",
    );
    expect(normalizeConsoleLine('[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO `users` ("roles") VALUES ($name)] 3μs')).toBe(
      "[<date>]-[datasource][Start]-[INSERT INTO users (roles) VALUES ($name)]",
    );
    expect(normalizeConsoleLine("count: 1234")).toBe("count: 1234");
    expect(normalizeConsoleLine("plain")).toBe("plain");
    expect(normalizeConsoleLine("[x] kept")).toBe("[x] kept");
    expect(dialectTranscript(["[2026-09-22T00:00:00.000Z]-[datasource][Request]-[{is_enabled: True}]"])).toEqual([
      "[<date>]-[datasource][Request]-[{is_enabled: 1}]",
    ]);
    expect(dialectTranscript([
      "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]",
      "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 5, rowsAffected: 1}]",
    ])).toEqual([
      "[<date>]-[datasource][Start]-[UPDATE users SET is_enabled = $is_enabled]",
      "[<date>]-[datasource][AfterCommand]-[{result: <id>, rowsAffected: 1}]",
    ]);
    expect(dialectTranscript([
      "[2026-09-22T00:00:00.000Z]-[datasource][Start]-[INSERT INTO users (name) VALUES ($name)]",
      "[2026-09-22T00:00:00.000Z]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]",
    ])[1]).toBe("[<date>]-[datasource][AfterCommand]-[{result: 1, rowsAffected: 1}]");
    const read = (provider: string) => readFileSync(join(shared, `${provider}.log`), "utf8").split("\n").filter((line) => line.length > 0);
    const logs = ["sqlite", "mysql", "postgres"].map((provider) => sqlText(read(provider)));
    expect(logs[1]).toEqual(logs[0]);
    expect(logs[2]).toEqual(logs[0]);
    const full = ["sqlite", "mysql", "postgres"].map((provider) => dialectTranscript(read(provider)));
    expect(full[1]).toEqual(full[0]);
    expect(full[2]).toEqual(full[0]);
  });

  it("matches the sqlite console transcript", async () => {
    const spec = parseYaml(readFileSync(join(shared, "cli.yaml"), "utf8")) as {
      now: string;
      steps: { argv: string[] }[];
    };
    const expected = readFileSync(join(shared, "sqlite.log"), "utf8").split("\n").filter((line) => line.length > 0).map(normalizeConsoleLine);
    const connection = new SqliteDriver().connection({ filename: ":memory:" });
    await connection.open();
    const command = connection.createCommand();
    await migrate(command);
    const now = () => new Date(spec.now);
    const middleware = new ExpectConsoleMiddleware(expected, now, normalizeConsoleLine);
    command.addMiddleware(middleware);
    const app = new App(command, now, () => "hash", () => new Uint8Array([1, 2, 3]), false);
    for (const step of spec.steps) await app.run(parse(step.argv));
    middleware.complete();
  });
});
