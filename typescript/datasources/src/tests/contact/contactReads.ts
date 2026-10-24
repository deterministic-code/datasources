import { NestedSelectBuilder } from "../../NestedSelectBuilder.ts";
import type { NestedSelectType } from "../../NestedSelectType.ts";
import { CreateNestedReadStoredProcedureStatement } from "../../statements/CreateNestedReadStoredProcedureStatement.ts";

export const contactGraph = (): NestedSelectType => {
  const source = { name: "ContactSource", key: "Id", properties: ["Id", "Name"], relations: [] };
  const type = { name: "ContactType", key: "Id", properties: ["Id", "Name"], relations: [] };
  const addressType = { name: "AddressType", key: "Id", properties: ["Id", "Name"], relations: [] };
  const phoneType = { name: "PhoneType", key: "Id", properties: ["Id", "Name"], relations: [] };
  const address = {
    name: "Address",
    key: "Id",
    properties: ["Id", "Line1", "City"],
    relations: [{ name: "AddressType", kind: "object" as const, from: "AddressTypeId", to: "Id", target: addressType }],
  };
  const phone = {
    name: "Phone",
    key: "Id",
    properties: ["Id", "Number"],
    relations: [{ name: "PhoneType", kind: "object" as const, from: "PhoneTypeId", to: "Id", target: phoneType }],
  };
  return {
    name: "Contact",
    key: "Id",
    properties: ["Id", "FirstName", "LastName", "Email"],
    relations: [
      { name: "ContactSource", kind: "object", from: "ContactSourceId", to: "Id", target: source },
      { name: "ContactType", kind: "object", from: "ContactTypeId", to: "Id", target: type },
      { name: "Addresses", kind: "list", from: "Id", to: "ContactId", target: address },
      { name: "Phones", kind: "list", from: "Id", to: "ContactId", target: phone },
    ],
  };
};

export const contactReads = {
  all: () => new NestedSelectBuilder().build(contactGraph()),
  withoutPhones: (emailType: string) =>
    new CreateNestedReadStoredProcedureStatement(
      "ReadContactsByEmail",
      new NestedSelectBuilder().build(contactGraph(), { stopAt: ["Phones"] }),
      [{ path: "Contact.Email", parameter: { name: "email", type: emailType } }],
    ),
};
