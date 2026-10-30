import type { ContactCard } from "../types/generated/views/contactCard";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class ContactCardRepository extends CRUDRepository<ContactCard> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactCard", entityMapper: new DictionaryMapper([["ContactCard", "contact_cards"]]), propertyMappings: new DictionaryMapper([["displayName", "display_name"]]), ids: [], createProperties: ["displayName"], updateProperties: ["displayName"], readProperties: ["displayName"] });
    this.createZeros = { displayName: "" };
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactCard>(connection, { entity: "ContactCard", entityMapper: new DictionaryMapper([["ContactCard", "contact_cards"]]), propertyMappings: new DictionaryMapper([["displayName", "display_name"]]), readProperties: ["displayName"] });
}
