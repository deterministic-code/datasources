import { describe, expect, it, vi } from "vitest";
import { CommandMiddlewareHost, ConsoleCommandMiddleware, formatElapsed } from "./index.ts";

describe("ConsoleCommandMiddleware", () => {
  it("formats elapsed time as ms or μs", () => {
    expect(formatElapsed(2)).toBe("2ms");
    expect(formatElapsed(1)).toBe("1ms");
    expect(formatElapsed(0.85)).toBe("850μs");
    expect(formatElapsed(0.4)).toBe("400μs");
  });

  it("logs Start and Finish with an injected clock", async () => {
    const lines: string[] = [];
    const stamps = [
      new Date("2026-08-28T01:54:12.101Z"),
      new Date("2026-08-28T01:54:12.103Z"),
    ];
    const middleware = new ConsoleCommandMiddleware(
      (line) => lines.push(line),
      () => stamps.shift() ?? new Date("2026-08-28T01:54:12.103Z"),
    );
    const host = new CommandMiddlewareHost();
    host.addMiddleware(middleware);
    await host.run('SELECT * FROM "widget" ORDER BY "id"', undefined, async () => {
      await new Promise((resolve) => setTimeout(resolve, 2));
      return { value: 1, rows: [] };
    });
    expect(lines[0]).toBe(
      '[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT * FROM "widget" ORDER BY "id"]',
    );
    expect(lines[1]).toMatch(
      /^\[2026-08-28T01:54:12.103Z\]-\[datasource\]\[Finish\]-\[SELECT \* FROM "widget" ORDER BY "id"\] \d+(ms|μs)$/,
    );
  });

  it("uses console.log and Date by default", async () => {
    const logged: string[] = [];
    const spy = vi.spyOn(console, "log").mockImplementation((line) => {
      logged.push(String(line));
    });
    const middleware = new ConsoleCommandMiddleware();
    middleware.beforeCommand("SELECT 1");
    middleware.afterCommand("SELECT 1", undefined, [], 2);
    expect(logged[0]).toMatch(
      /^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z\]-\[datasource\]\[Start\]-\[SELECT 1\]$/,
    );
    expect(logged[1]).toMatch(/\[Finish\]-\[SELECT 1\] 2ms$/);
    spy.mockRestore();
  });

  it("trims trailing newlines before logging", async () => {
    const lines: string[] = [];
    const host = new CommandMiddlewareHost();
    host.addMiddleware(new ConsoleCommandMiddleware((line) => lines.push(line)));
    await host.run("SELECT 1\n", undefined, async () => ({ value: 1, rows: [] }));
    expect(lines[0].replace(/^\[[^\]]+\]-/, "")).toBe("[datasource][Start]-[SELECT 1]");
  });

  it("logs Request and AfterCommand when isVerbose", async () => {
    const lines: string[] = [];
    const stamps = [
      new Date("2026-08-28T01:54:12.101Z"),
      new Date("2026-08-28T01:54:12.102Z"),
      new Date("2026-08-28T01:54:12.103Z"),
      new Date("2026-08-28T01:54:12.104Z"),
    ];
    const host = new CommandMiddlewareHost();
    host.addMiddleware(
      new ConsoleCommandMiddleware(
        (line) => lines.push(line),
        () => stamps.shift() ?? new Date("2026-08-28T01:54:12.104Z"),
        { isVerbose: "true" },
      ),
    );
    await host.run("SELECT 1", [{ name: "id", value: 1 }], async () => ({
      value: 1,
      rows: [{ id: 1 }],
    }));
    expect(lines).toHaveLength(4);
    expect(lines[0]).toBe("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]");
    expect(lines[1]).toBe("[2026-08-28T01:54:12.102Z]-[datasource][Request]-[{id: 1}]");
    expect(lines[2]).toMatch(
      /^\[2026-08-28T01:54:12.103Z\]-\[datasource\]\[Finish\]-\[SELECT 1\] \d+(ms|μs)$/,
    );
    expect(lines[3]).toBe("[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{id: 1}]");
    lines.length = 0;
    await host.run("DELETE FROM user", [{ name: "id", value: 2 }], async () => ({
      value: { result: null, rowsAffected: 1 },
      rows: [],
    }));
    expect(lines[3]).toBe(
      "[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{result: null, rowsAffected: 1}]",
    );
  });

  it("serializes Date, binary, and bigint in verbose bodies", () => {
    const lines: string[] = [];
    const middleware = new ConsoleCommandMiddleware(
      (line) => lines.push(line),
      () => new Date("2026-08-28T01:54:12.101Z"),
      { isVerbose: "1" },
    );
    middleware.beforeCommand("SELECT 1");
    expect(lines[1]).toBe("[2026-08-28T01:54:12.101Z]-[datasource][Request]-[{}]");
    lines.length = 0;
    middleware.beforeCommand("SELECT 1", [
      { name: "when", value: new Date("2026-08-28T01:54:12.000Z") },
      { name: "blob", value: new Uint8Array([1, 2, 3]) },
      { name: "n", value: 1n },
    ]);
    middleware.afterCommand(
      "SELECT 1",
      undefined,
      [
        {
          when: new Date("2026-08-28T01:54:12.000Z"),
          blob: new Uint8Array([1, 2, 3]),
          n: 1n,
        },
      ],
      2,
    );
    expect(lines[1]).toBe(
      "[2026-08-28T01:54:12.101Z]-[datasource][Request]-[{when: 2026-08-28T01:54:12.000Z, blob: AQID, n: 1}]",
    );
    expect(lines[3]).toBe(
      "[2026-08-28T01:54:12.101Z]-[datasource][AfterCommand]-[{when: 2026-08-28T01:54:12.000Z, blob: AQID, n: 1}]",
    );
  });
});
