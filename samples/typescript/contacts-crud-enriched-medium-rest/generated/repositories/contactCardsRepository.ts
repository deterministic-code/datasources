import { ContactCard } from "../types/view/contactCard";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class ContactCardsRepository extends CRUDRepository<ContactCard> {
  constructor(connection: IConnection) {
    super(connection, { type: ContactCard, entity: "ContactCard", entityMapper: new DictionaryMapper([["ContactCard", "contact_cards"]]), propertyMappings: new DictionaryMapper([["displayName", "display_name"]]), ids: [], createProperties: ["displayName"], updateProperties: ["displayName"], readProperties: ["displayName"], schema: new Schema({ displayName: { type: "string", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactCard>(connection, { type: ContactCard, entity: "ContactCard", entityMapper: new DictionaryMapper([["ContactCard", "contact_cards"]]), propertyMappings: new DictionaryMapper([["displayName", "display_name"]]), readProperties: ["displayName"], schema: new Schema({ displayName: { type: "string", nullable: false, hasDefault: false } }) });
}
