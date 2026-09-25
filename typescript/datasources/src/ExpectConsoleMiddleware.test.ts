import { describe, expect, it } from "vitest";
import { CommandMiddlewareHost } from "./CommandMiddlewareHost.ts";
import { ExpectConsoleMiddleware, consoleSignature } from "./ExpectConsoleMiddleware.ts";

const now = () => new Date("2026-08-28T01:54:12.101Z");

describe("ExpectConsoleMiddleware", () => {
  it("matches the transcript and drops the finish duration", async () => {
    const middleware = new ExpectConsoleMiddleware(
      [
        "[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]",
        "[2026-08-28T01:54:12.101Z]-[datasource][Request]-[{id: 1}]",
        "[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]",
        "[2026-08-28T01:54:12.101Z]-[datasource][AfterCommand]-[{id: 1}]",
      ],
      now,
    );
    const host = new CommandMiddlewareHost();
    host.addMiddleware(middleware);
    await host.run("SELECT 1", [{ name: "id", value: 1 }], async () => ({
      value: 1,
      rows: [{ id: 1 }],
    }));
    middleware.complete();
    expect(consoleSignature("[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1] 2ms")).toBe(
      "[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]",
    );
    expect(consoleSignature("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]")).toBe(
      "[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]",
    );
    expect(consoleSignature("[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1] 3μs")).toBe(
      "[2026-08-28T01:54:12.101Z]-[datasource][Finish]-[SELECT 1]",
    );
  });

  it("rejects a different line and a short transcript", async () => {
    const host = new CommandMiddlewareHost();
    const mismatch = new ExpectConsoleMiddleware(["[nope]"], now);
    host.addMiddleware(mismatch);
    await expect(
      host.run("SELECT 1", [{ name: "id", value: 1 }], async () => ({
        value: 1,
        rows: [{ id: 1 }],
      })),
    ).rejects.toThrow(/console mismatch at 0/);
    expect(() => new ExpectConsoleMiddleware(["[missing]"], now).complete()).toThrow(
      /actual:   <missing>/,
    );
    const extraHost = new CommandMiddlewareHost();
    extraHost.addMiddleware(new ExpectConsoleMiddleware(
      ["[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]"],
      now,
    ));
    await expect(
      extraHost.run("SELECT 1", undefined, async () => ({ value: null, rows: [] })),
    ).rejects.toThrow(/<missing>/);
  });
});
