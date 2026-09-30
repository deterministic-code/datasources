import { Operator, Where } from "../../statements/index.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import type { Constructable } from "../../converters/Constructable.ts";
import { AddressType } from "./AddressType.ts";
import { columns, snake } from "./contactColumns.ts";
import type { ContactLookups } from "./contactLookups.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { PhoneType } from "./PhoneType.ts";

const of = async <T extends { Id: number }>(
  connection: IConnection,
  type: Constructable<T>,
  name: string,
): Promise<number> => {
  const row = await new CRUDRepository(connection, {
    type,
    entityMapper: snake,
    columnMappings: columns,
    ids: ["Id"],
  }).readSingle(new Where(new Operator.Equals("Name", name)));
  if (!row) throw new Error(`${type.name} '${name}' missing`);
  return row.Id;
};

export const contactLookupIds = async (connection: IConnection): Promise<ContactLookups> => ({
  ContactSourceId: await of(connection, ContactSource, "Web"),
  ContactTypeId: await of(connection, ContactType, "Person"),
  AddressTypeId: await of(connection, AddressType, "Home"),
  PhoneTypeId: await of(connection, PhoneType, "Mobile"),
});
