import { ExpectConsoleMiddleware } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseYaml } from "yaml";
import { describe, it } from "vitest";
import { App } from "./app.ts";
import { migrate } from "./migrate.ts";
import { parse } from "./parse.ts";

const shared = resolve(dirname(fileURLToPath(import.meta.url)), "../../../user-roles-simple-cli-shared");

describe("console transcript", () => {
  it("matches the sqlite console transcript", async () => {
    const spec = parseYaml(readFileSync(join(shared, "cli.yaml"), "utf8")) as {
      now: string;
      steps: { argv: string[] }[];
    };
    const expected = readFileSync(join(shared, "sqlite.log"), "utf8").split("\n").filter((line) => line.length > 0);
    const connection = new SqliteDriver().connection({ filename: ":memory:" });
    await connection.open();
    const command = connection.createCommand();
    await migrate(command);
    const now = () => new Date(spec.now);
    const middleware = new ExpectConsoleMiddleware(expected, now);
    command.addMiddleware(middleware);
    const app = new App(command, now, () => "hash", () => new Uint8Array([1, 2, 3]));
    for (const step of spec.steps) await app.run(parse(step.argv));
    middleware.complete();
  });
});
