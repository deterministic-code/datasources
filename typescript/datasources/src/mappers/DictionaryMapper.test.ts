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
      ["FirstName", "given_name"],
      ["LastName", "family_name"],
      ["Email", "email_address"],
    ]);
    const firstName = mapper.map("FirstName");
    const lastName = mapper.map("LastName");
    const email = mapper.map("Email");
    const phone = mapper.map("Phone");
    const fromFirst = mapper.reverseMap("given_name");
    const fromLast = mapper.reverseMap("family_name");
    const fromEmail = mapper.reverseMap("email_address");
    const fromPhone = mapper.reverseMap("Phone");
    expect(firstName).toBe("given_name");
    expect(lastName).toBe("family_name");
    expect(email).toBe("email_address");
    expect(phone).toBe("Phone");
    expect(fromFirst).toBe("FirstName");
    expect(fromLast).toBe("LastName");
    expect(fromEmail).toBe("Email");
    expect(fromPhone).toBe("Phone");
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
