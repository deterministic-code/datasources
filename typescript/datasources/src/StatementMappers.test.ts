import { describe, expect, it } from "vitest";
import { DictionaryMapper } from "./mappers/DictionaryMapper.ts";
import { StatementMappers } from "./StatementMappers.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import { Identifier } from "./statements/Identifier.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { Join } from "./statements/Join.ts";
import { Operator } from "./statements/Operator.ts";
import { OrderBy } from "./statements/OrderBy.ts";
import { SelectInsertedIdStatement } from "./statements/SelectInsertedIdStatement.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { Where } from "./statements/Where.ts";
import { WhereText } from "./statements/WhereText.ts";
import type { Cell } from "./converters/Cell.ts";

const mapper = (name: string, value: Cell): Cell =>
  typeof value === "string" ? `${name}:${value.toUpperCase()}` : value;

describe("StatementMappers", () => {
  it("maps a select", () => {
    const mappings = new StatementMappers(
      new DictionaryMapper([
        ["User", "people"],
        ["Role", "roles"],
      ]),
      new DictionaryMapper([
        ["Name", "full_name"],
        ["title", "book.title"],
        ["id", "user_id"],
      ]),
    );
    const select = new SelectStatement({
      distinct: true,
      table: "User",
      fields: ["Name", "title", "scope.title", "alias.other", '"schema.name".id'],
      where: new Where(new Operator.Equals("Name", "Bea")),
      joins: [Join.inner("Role", "User.id", "Role.user_id"), Join.cross("logs")],
      groupBy: ["Name"],
      having: new Where(new Operator.Equals("Name", "Bea")),
      orderBy: [OrderBy.desc("Name")],
      limit: 5,
      offset: 1,
    });
    const mapped = mappings.apply(select, mapper) as SelectStatement;
    expect(mapped.table).toBe("people");
    expect(mapped.fields).toEqual([
      "full_name",
      "book.title",
      "book.title",
      "alias.other",
      '"schema.name".user_id',
    ]);
    expect(mapped.distinct).toBe(true);
    expect(mapped.limit).toBe(5);
    expect(mapped.offset).toBe(1);
    expect(mapped.groupBy).toEqual(["full_name"]);
    expect(mapped.orderBy).toEqual([{ column: "full_name", direction: "desc" }]);
    expect((mapped.joins[0]?.on as Operator.Equals).left).toBeInstanceOf(Identifier);
    expect(((mapped.joins[0]?.on as Operator.Equals).left as Identifier).name).toBe("people.user_id");
    expect(mapped.joins[1]).toEqual({ kind: "cross", table: "logs" });
    expect(mapped.getText()).toContain("full_name = $full_name");
    expect(mapped.where?.parameters?.[0]?.value).toBe("Name:BEA");
    expect(mappings.property("full_name")).toBe("Name");
    expect(mappings.property("people.full_name")).toBe("Name");
    expect(mappings.property("other")).toBe("other");
    expect(mappings.property("people.missing")).toBe("people.missing");
  });

  it("maps writes", () => {
    const mappings = new StatementMappers(
      new DictionaryMapper([
        ["User", "people"],
        ["Role", "roles"],
      ]),
      new DictionaryMapper([
        ["Name", "full_name"],
        ["title", "book.title"],
        ["id", "user_id"],
      ]),
    );
    const insert = mappings.apply(new InsertStatement("User", [{ name: "Name", value: "Ada" }]), mapper) as InsertStatement;
    expect(insert.getText()).toBe("INSERT INTO people (full_name) VALUES ($full_name)");
    expect(insert.fields[0]?.value).toBe("Ada");

    const update = mappings.apply(
      new UpdateStatement("User", [{ name: "Name", value: "Ada" }], new Where(new Operator.Equals("Name", "Bea"))),
      mapper,
    ) as UpdateStatement;
    expect(update.getText()).toBe(
      "UPDATE people SET full_name = $full_name WHERE full_name = $full_name__where",
    );
    expect(update.parameters[0]?.value).toBe("Ada");
    expect(update.parameters[1]).toEqual({ name: "full_name__where", value: "Name:BEA" });

    const deleted = mappings.apply(
      new DeleteStatement("User", new Where(new Operator.Equals("Name", "Bea"))),
      mapper,
    ) as DeleteStatement;
    expect(deleted.getText()).toBe("DELETE FROM people WHERE full_name = $full_name");

    const inserted = mappings.apply(new SelectInsertedIdStatement("User", "id"), mapper) as SelectInsertedIdStatement;
    expect(inserted.table).toBe("people");
    expect(inserted.column).toBe("user_id");
  });

  it("leaves raw and other statements", () => {
    const mappings = new StatementMappers(
      new DictionaryMapper([
        ["User", "people"],
        ["Role", "roles"],
      ]),
      new DictionaryMapper([
        ["Name", "full_name"],
        ["title", "book.title"],
        ["id", "user_id"],
      ]),
    );
    const rawInsert = new InsertStatement("INSERT INTO t VALUES (1)");
    expect(mappings.apply(rawInsert, mapper)).toBe(rawInsert);
    const rawUpdate = new UpdateStatement("x");
    expect(mappings.apply(rawUpdate, mapper)).toBe(rawUpdate);
    expect(rawUpdate.parameters).toEqual([]);
    const rawDelete = DeleteStatement.raw("DELETE FROM t");
    expect(mappings.apply(rawDelete, mapper)).toBe(rawDelete);
    const bare = new SelectStatement("SELECT 1");
    expect(mappings.apply(bare, mapper)).toBe(bare);
    const call = new ExecuteStoredProcedureStatement("CALL save_user()");
    expect(mappings.apply(call, mapper)).toBe(call);
    expect(mappings.apply(undefined, mapper)).toBeUndefined();
    const text = new WhereText("name = $name");
    expect(mappings.applyWhere(text, mapper)).toBe(text);
    const kept = mappings.apply(new DeleteStatement("User", text), mapper) as DeleteStatement;
    expect(kept.where).toBe(text);
    expect(kept.getText()).toBe("DELETE FROM people WHERE name = $name");
  });

  it("maps a contacts select", () => {
    const mappings = new StatementMappers(
      new DictionaryMapper([
        ["Contact", "contacts_base"],
        ["ContactSource", "contact_source"],
        ["ContactType", "contact_type"],
        ["Address", "address"],
        ["AddressType", "address_type"],
        ["Phone", "phone"],
        ["PhoneType", "phone_type"],
      ]),
      new DictionaryMapper([
        ["Id", "id"],
        ["FirstName", "first_name"],
        ["LastName", "last_name"],
        ["Email", "email"],
        ["Name", "name"],
        ["Line1", "line1"],
        ["City", "city"],
        ["Number", "number"],
        ["ContactId", "contact_id"],
        ["ContactSourceId", "contact_source_id"],
        ["ContactTypeId", "contact_type_id"],
        ["AddressTypeId", "address_type_id"],
        ["PhoneTypeId", "phone_type_id"],
      ]),
    );
    const select = new SelectStatement({
      distinct: true,
      table: "Contact",
      fields: [
        "Contact.FirstName",
        "Contact.LastName",
        "Contact.Email",
        "ContactSource.Name",
        "Address.Line1",
        "Address.City",
        "Phone.Number",
      ],
      where: new Where(
        new Operator.And(
          new Operator.Equals("Contact.Email", "ada@ex.com"),
          new Operator.GreaterThan("Address.City", "A"),
        ),
      ),
      joins: [
        Join.inner("ContactSource", "Contact.ContactSourceId", "ContactSource.Id"),
        Join.inner("ContactType", "Contact.ContactTypeId", "ContactType.Id"),
        Join.left("Address", "Contact.Id", "Address.ContactId"),
        Join.left("AddressType", "Address.AddressTypeId", "AddressType.Id"),
        Join.left("Phone", "Contact.Id", "Phone.ContactId"),
        Join.left("PhoneType", "Phone.PhoneTypeId", "PhoneType.Id"),
      ],
      groupBy: ["Contact.LastName"],
      having: new Where(new Operator.Equals("ContactType.Name", "person")),
      orderBy: [OrderBy.asc("Contact.LastName"), OrderBy.desc("Contact.FirstName")],
      limit: 20,
      offset: 1,
    });
    const mapped = mappings.apply(select, mapper) as SelectStatement;
    expect(mapped.getText()).toBe(
      "SELECT DISTINCT contacts_base.first_name, contacts_base.last_name, contacts_base.email, contact_source.name, address.line1, address.city, phone.number FROM contacts_base INNER JOIN contact_source ON contacts_base.contact_source_id = contact_source.id INNER JOIN contact_type ON contacts_base.contact_type_id = contact_type.id LEFT JOIN address ON contacts_base.id = address.contact_id LEFT JOIN address_type ON address.address_type_id = address_type.id LEFT JOIN phone ON contacts_base.id = phone.contact_id LEFT JOIN phone_type ON phone.phone_type_id = phone_type.id WHERE contacts_base.email = $contacts_base.email AND address.city > $address.city GROUP BY contacts_base.last_name HAVING contact_type.name = $contact_type.name ORDER BY contacts_base.last_name ASC, contacts_base.first_name DESC LIMIT 20 OFFSET 1",
    );
    expect(mapped.where?.parameters[0]?.value).toBe("Contact.Email:ADA@EX.COM");
    expect(mapped.where?.parameters[1]?.value).toBe("Address.City:A");
    expect((mapped.having as Where).parameters[0]?.value).toBe("ContactType.Name:PERSON");
  });
});
