import { describe, expect, it } from "vitest";
import {
  MockCommand,
  Native,
  MockConnection,
  MockReader,
  MetaDataRowTypeConverter,
  TransformMapper,
} from "./index.ts";

class NameValue {
  name = "";
  value = "";
}

class Contact {
  name = "";
}

class User {
  id = 0;
  username = "";
  email = "";
  contact_name1 = "";
  contact_name2 = "";
  contacts: Contact[] = [];
}

const userContactsMapper = () =>
  new TransformMapper<User, User>(
    (row) => {
      row.contacts = [
        Object.assign(new Contact(), { name: row.contact_name1 }),
        Object.assign(new Contact(), { name: row.contact_name2 }),
      ];
      return row;
    },
    (user) => {
      user.contact_name1 = user.contacts[0]?.name ?? "";
      user.contact_name2 = user.contacts[1]?.name ?? "";
      return user;
    },
  );

describe("Command", () => {
  it("executeReader returns NameValue objects", async () => {
    const command = new MockCommand(new MockConnection());
    command.reader = new MockReader([{ name: "theme", value: "dark" }]);
    const reader = await command.executeReader("SELECT name, value FROM settings");
    const converter = new MetaDataRowTypeConverter<NameValue>();
    const items: NameValue[] = [];
    while (await reader.next()) {
      items.push(converter.convert(reader.current!, NameValue));
    }
    expect(items[0]?.name).toBe("theme");
    expect(items[0]?.value).toBe("dark");
  });

  it("execute insert update and delete", async () => {
    const command = new MockCommand(new MockConnection());
    await command.executeNative(new Native("insert"), [
      { name: "id", value: 1 },
      { name: "name", value: "a" },
    ]);
    await command.executeNative(new Native("update"), [
      { name: "id", value: 1 },
      { name: "name", value: "A" },
    ]);
    await command.executeNative(new Native("delete"), [{ name: "id", value: 1 }]);
    expect(command.executeCalls.map((call) => call.text)).toEqual([
      "insert",
      "update",
      "delete",
    ]);
    expect(command.executeCalls[0]?.parameters?.map((parameter) => parameter.name)).toEqual([
      "id",
      "name",
    ]);
  });

  it("executeReader maps flat contacts", async () => {
    const command = new MockCommand(new MockConnection());
    command.reader = new MockReader([
      {
        id: 1,
        username: "bobby",
        email: "bob@asdf.com",
        contact_name1: "bob",
        contact_name2: "roberta",
      },
    ]);
    const reader = await command.executeReader(
      "SELECT contact_name1, contact_name2, email, id, username FROM users",
    );
    const converter = new MetaDataRowTypeConverter<User>();
    const mapper = userContactsMapper();
    let user: User | undefined;
    while (await reader.next()) {
      user = mapper.map(converter.convert(reader.current!, User));
    }
    expect(user?.username).toBe("bobby");
    expect(user?.email).toBe("bob@asdf.com");
    expect(user?.contacts.map((contact) => contact.name)).toEqual(["bob", "roberta"]);
  });
});
