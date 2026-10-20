import type { ContactPreferences } from "../types/generated/datasource/contactPreferences.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class ContactPreferencesRepository extends CRUDRepository<ContactPreferences> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactPreferences", entityMapper: new DictionaryMapper([["ContactPreferences", "contact_preferences"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "locale", "timezone"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactPreferences>(connection, { entity: "ContactPreferences", entityMapper: new DictionaryMapper([["ContactPreferences", "contact_preferences"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "locale", "timezone"] });
}
