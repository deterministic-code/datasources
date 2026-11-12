import { describe, expect, it } from "vitest";
import {
  ArrayReader,
  CommandMiddlewareHost,
  InsertStatement,
  Native,
  Response,
  ReturnStatement,
  SelectStatement,
} from "./index.ts";
import type { Cells } from "./converters/Value.ts";
import type { IStatement } from "./statements/IStatement.ts";
import type { Parameter } from "./Parameter.ts";

class CellHost extends CommandMiddlewareHost {
  rows: Cells[] = [];
  natives = 0;

  executeReaderNative = async (): Promise<ArrayReader> => new ArrayReader(this.rows);

  executeNative = async (): Promise<Response> => {
    this.natives += 1;
    return new Response(null, 3);
  };

  executeStatement = (statement: IStatement, parameters?: Parameter[]): Promise<Response> =>
    this.readsCell(statement)
      ? this.firstCell(statement, parameters)
      : this.executeNative(new Native(""), parameters);
}

describe("CommandMiddlewareHost", () => {
  it("base execute methods require a driver", () => {
    const host = new CommandMiddlewareHost();
    expect(() => host.execute("SELECT id FROM users")).toThrow("executeNative");
    expect(() => host.execute("  ")).toThrow("empty command");
    expect(() => host.executeReader("SELECT id FROM users")).toThrow("executeReaderNative");
    expect(() => host.executeReader("")).toThrow("empty command");
  });

  it("rewrites the text and runs afterCommand", async () => {
    const host = new CommandMiddlewareHost();
    const seen: string[] = [];
    let used = "";
    host.addMiddleware({
      beforeCommand: (text) => {
        seen.push(`before:${text}`);
        return { text: "SELECT 2" };
      },
      afterCommand: (text, _params, results, elapsedMs) => {
        seen.push(`after:${text}:${results.length}:${elapsedMs >= 0}`);
      },
    });
    const result = await host.run("SELECT 1", undefined, async (text) => {
      used = text;
      seen.push("op");
      return { value: 7, rows: [{ id: 1 }] };
    });
    expect(result).toBe(7);
    expect(used).toBe("SELECT 2");
    expect(seen[0]).toBe("before:SELECT 1");
    expect(seen[1]).toBe("op");
    expect(seen[2]?.startsWith("after:SELECT 2:1:true")).toBe(true);
  });

  it("runs with no middleware", async () => {
    const host = new CommandMiddlewareHost();
    expect(await host.run("", undefined, async () => ({ value: "ok", rows: [] }))).toBe(
      "ok",
    );
  });

  it("skips the command when beforeCommand throws", async () => {
    const host = new CommandMiddlewareHost();
    let ran = false;
    host.addMiddleware({
      beforeCommand: () => {
        throw new Error("blocked");
      },
      afterCommand: () => {
        ran = true;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        ran = true;
        return { value: 1, rows: [] };
      }),
    ).rejects.toThrow("blocked");
    expect(ran).toBe(false);
  });

  it("runs afterCommand with error when the command fails", async () => {
    const host = new CommandMiddlewareHost();
    let captured: Error | undefined;
    host.addMiddleware({
      beforeCommand: () => undefined,
      afterCommand: (_q, _p, _r, _e, error) => {
        captured = error;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        throw new Error("fail-op");
      }),
    ).rejects.toThrow("fail-op");
    expect(captured?.message).toBe("fail-op");
  });

  it("wraps a non-Error throw for afterCommand", async () => {
    const host = new CommandMiddlewareHost();
    let captured: Error | undefined;
    host.addMiddleware({
      beforeCommand: () => undefined,
      afterCommand: (_q, _p, _r, _e, error) => {
        captured = error;
      },
    });
    await expect(
      host.run("select", undefined, async () => {
        throw "string-fail";
      }),
    ).rejects.toBe("string-fail");
    expect(captured?.message).toBe("string-fail");
  });

  it("firstCell reads the first row first column", async () => {
    const host = new CellHost();
    host.rows = [{ id: 2, name: "bea" }];
    expect(await host.executeStatement(new SelectStatement({ table: "items", fields: ["id"] }))).toEqual({
      result: 2,
      rowsAffected: 1,
      parameters: undefined,
    });
    expect(
      await host.executeStatement(new InsertStatement("items", [{ name: "name", value: "bea" }], "id")),
    ).toEqual({
      result: null,
      rowsAffected: 3,
    });
    const x = { name: "x", value: -3 };
    expect(await host.executeStatement(new ReturnStatement("abs", [x]), [x])).toEqual({
      result: 2,
      rowsAffected: 1,
      parameters: [x],
    });
    host.rows = [];
    expect(await host.executeStatement(new SelectStatement({ table: "items", fields: ["id"] }))).toEqual({
      result: null,
      rowsAffected: 0,
      parameters: undefined,
    });
    host.rows = [{}];
    expect(await host.executeStatement(new SelectStatement({ table: "items", fields: ["id"] }))).toEqual({
      result: null,
      rowsAffected: 1,
      parameters: undefined,
    });
    expect(await host.executeStatement(new InsertStatement("items", []))).toEqual({
      result: null,
      rowsAffected: 3,
    });
    expect(host.natives).toBe(2);
  });
});
