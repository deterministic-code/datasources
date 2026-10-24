import type { ContactPreferences } from "../types/generated/datasource/contactPreferences";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactPreferences>()
    .add({ entity: "ContactPreferences", entityMapper: new DictionaryMapper([["ContactPreferences", "contact_preferences"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["uuid", "version", "contactId", "key", "locale", "timezone"], updateProperties: ["uuid", "version", "contactId", "key", "locale", "timezone"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "locale", "timezone"] })
    .add("mysql", { entity: "ContactPreferences", entityMapper: new DictionaryMapper([["ContactPreferences", "contact_preferences"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["contactId", "key", "locale", "timezone"], updateProperties: ["uuid", "version", "contactId", "key", "locale", "timezone"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "locale", "timezone"] })
    .add("postgres", { entity: "ContactPreferences", entityMapper: new DictionaryMapper([["ContactPreferences", "contact_preferences"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["contactId", "key", "locale", "timezone"], updateProperties: ["uuid", "version", "contactId", "key", "locale", "timezone"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "locale", "timezone"] })
    .for(connection);

export class ContactPreferencesRepository extends CRUDRepository<ContactPreferences> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactPreferences>(connection, options(connection));
}
