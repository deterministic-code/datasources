import { describe, expect, it } from "vitest";
import { NestedSelectBuilder } from "./NestedSelectBuilder.ts";
import { DefaultStatementTextRenderer } from "./DefaultStatementTextRenderer.ts";
import { CreateNestedReadStoredProcedureStatement } from "./CreateNestedReadStoredProcedureStatement.ts";
import { Operator } from "./Operator.ts";
import { Where } from "./Where.ts";
import { contactGraph, contactReads } from "../tests/contact/contactReads.ts";

describe("NestedSelectBuilder", () => {
  it("builds contact joins and a type map", () => {
    const nested = contactReads.all();
    expect(nested.select.table).toBe("Contact");
    expect(nested.select.fields).toEqual([
      "Contact.Id",
      "Contact.FirstName",
      "Contact.LastName",
      "Contact.Email",
      "Contact.ContactSource.Id",
      "Contact.ContactSource.Name",
      "Contact.ContactType.Id",
      "Contact.ContactType.Name",
      "Contact.Address.Id",
      "Contact.Address.Line1",
      "Contact.Address.City",
      "Contact.Address.AddressType.Id",
      "Contact.Address.AddressType.Name",
      "Contact.Phone.Id",
      "Contact.Phone.Number",
      "Contact.Phone.PhoneType.Id",
      "Contact.Phone.PhoneType.Name",
    ]);
    expect(nested.select.joins).toHaveLength(6);
    expect(nested.map.key).toBe("Id");
    expect((nested.map.fields.Addresses as { key: string }).key).toBe("AddressId");
    expect((nested.map.fields.Phones as { key: string }).key).toBe("PhoneId");
  });

  it("stops at phones", () => {
    const nested = new NestedSelectBuilder().build(contactGraph(), { stopAt: ["Phones"] });
    expect(nested.select.fields.some((field) => field.includes("Phone"))).toBe(false);
    expect(nested.select.joins.some((join) => join.table.includes("Phone"))).toBe(false);
    expect(nested.map.fields.Phones).toBeUndefined();
    expect(nested.map.fields.Addresses).toBeDefined();
  });

  it("adds a procedure where argument", () => {
    const procedure = contactReads.withoutPhones("varchar");
    expect(procedure.name).toBe("ReadContactsByEmail");
    expect(procedure.arguments[0]?.path).toBe("Contact.Email");
    const sql = DefaultStatementTextRenderer.instance.render(procedure.select);
    expect(sql).toContain("Contact.Email");
    expect(sql).not.toContain("Phone");
    expect(DefaultStatementTextRenderer.instance.render(procedure)).toBe("");
  });

  it("ands a procedure argument onto an existing where", () => {
    const nested = new NestedSelectBuilder().build(contactGraph(), {
      stopAt: ["Phones"],
      where: new Where(new Operator.Equals("Contact.Id", 1)),
    });
    const procedure = new CreateNestedReadStoredProcedureStatement("ReadContactsByEmail", nested, [
      { path: "Contact.Email", parameter: { name: "email", type: "varchar" } },
    ]);
    expect(DefaultStatementTextRenderer.instance.render(procedure.select)).toContain(" AND ");
  });
});
