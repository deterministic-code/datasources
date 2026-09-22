import { Operator, WhereClause } from "../../statements/index.ts";
import type { IConnection } from "../../IConnection.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import type { Constructable } from "../../converters/Constructable.ts";
import { AddressType } from "./AddressType.ts";
import type { ContactLookups } from "./contactLookups.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { PhoneType } from "./PhoneType.ts";

const of = async <T extends { id: number; name: string }>(
  connection: IConnection,
  type: Constructable<T>,
  entity: string,
  name: string,
): Promise<number> => {
  const row = await new CRUDRepository({ type, entity, connection, columns: ["id", "name"] }).readSingle(
    new WhereClause(new Operator.Equals("name", name)),
  );
  if (!row) throw new Error(`${entity} '${name}' missing`);
  return row.id;
};

export const contactLookupIds = async (connection: IConnection): Promise<ContactLookups> => ({
  contact_source_id: await of(connection, ContactSource, "contact_source", "Web"),
  contact_type_id: await of(connection, ContactType, "contact_type", "Person"),
  address_type_id: await of(connection, AddressType, "address_type", "Home"),
  phone_type_id: await of(connection, PhoneType, "phone_type", "Mobile"),
});
