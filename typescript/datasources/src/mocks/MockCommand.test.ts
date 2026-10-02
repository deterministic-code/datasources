import { describe, expect, it } from "vitest";
import {
  execute,
  executeReader,
  ExecuteStoredProcedureStatement,
  Native,
  MockCommand,
  MockConnection,
  MockReader,
} from "../index.ts";

describe("MockCommand", () => {
  it("records command execute paths", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    command.response = { result: 1, rowsAffected: 2 };
    command.reader = new MockReader([{ id: 1 }]);
    expect(await command.executeNative(new Native("insert"), [{ name: "id", value: 1 }])).toEqual({
      result: 1,
      rowsAffected: 2,
    });
    expect(command.executeCalls).toHaveLength(1);
    const reader = await command.executeReaderNative(new Native(""));
    expect(reader).toBe(command.reader);

    command.reader = {
      current: undefined,
      next: async () => false,
      nextResult: async () => false,
      async *read() {
        while (await this.next()) {
          if (this.current !== undefined) yield this.current;
        }
      },
      dispose: async () => {},
    };
    expect(await command.executeReaderNative(new Native("plain"))).toBe(command.reader);

    command.executeError = new Error("exec");
    await expect(command.executeNative(new Native(""))).rejects.toThrow("exec");
    command.executeReaderError = new Error("read");
    await expect(command.executeReaderNative(new Native("x"))).rejects.toThrow("read");

    command.executeError = undefined;
    command.executeReaderError = undefined;
    const fields = [{ name: "id", value: 1 }];
    const statement = new ExecuteStoredProcedureStatement("save", fields);
    await execute(command, statement, fields);
    expect(command.executeCalls.at(-1)).toEqual({
      text: "CALL save($id)",
      parameters: fields,
    });
    await executeReader(command, statement, fields);
    expect(command.executeReaderCalls.at(-1)).toEqual({
      text: "CALL save($id)",
      parameters: fields,
    });
  });

  it("runs command middleware on mock commands", async () => {
    const connection = new MockConnection();
    const command = new MockCommand(connection);
    const seen: string[] = [];
    command.addMiddleware({
      beforeCommand: (text) => {
        seen.push(`before:${text}`);
      },
      afterCommand: (text) => {
        seen.push(`after:${text}`);
      },
    });
    await command.executeNative(new Native("insert"));
    expect(seen).toEqual(["before:insert", "after:insert"]);
  });
});
