import { describe, expect, it } from "vitest";
import { DictionaryMapper } from "./DictionaryMapper.ts";
import { MockCommand } from "../mocks/mock-command.ts";
import { MockConnection } from "../mocks/mock-connection.ts";
import { MockReader } from "../mocks/mock-reader.ts";
import { ReadonlyRepository } from "../ReadonlyRepository.ts";

class Account {
  Key = 0;
  Login = "";
}

describe("DictionaryMapper", () => {
  it("maps known names and passes through the rest", () => {
    const mapper = new DictionaryMapper([
      ["Key", "id"],
      ["Login", "username"],
    ]);
    expect(mapper.map("Key")).toBe("id");
    expect(mapper.reverseMap("id")).toBe("Key");
    expect(mapper.map("other")).toBe("other");
    expect(mapper.reverseMap("other")).toBe("other");
  });

  it("read reverse-maps column names", async () => {
    const connection = new MockConnection(undefined, "memory");
    const command = new MockCommand(connection);
    connection.command = command;
    const repo = new ReadonlyRepository(connection, {
      type: Account,
      entity: "user",
      columnMappings: new DictionaryMapper([
        ["Key", "id"],
        ["Login", "username"],
      ]),
    });
    command.reader = new MockReader([{ id: 7, username: "ada" }]);
    const row = await repo.readSingle();
    expect(row?.Key).toBe(7);
    expect(row?.Login).toBe("ada");
    expect(command.executeReaderCalls[0]?.text).toBe("SELECT id, username FROM user");
  });
});
