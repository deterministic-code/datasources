import type { ContactSettings } from "../types/generated/datasource/contactSettings";
import { CRUDRepository, CRUDRepositoryBuilder, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

const options = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactSettings>()
    .add({ entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["uuid", "created", "updated", "version", "contactId", "key", "value"], updateProperties: ["uuid", "created", "updated", "version", "contactId", "key", "value"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("sqlite", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["uuid", "version", "contactId", "key", "value"], updateProperties: ["uuid", "version", "contactId", "key", "value"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("mysql", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["contactId", "key", "value"], updateProperties: ["uuid", "version", "contactId", "key", "value"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("postgres", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: ["id"], createProperties: ["contactId", "key", "value"], updateProperties: ["uuid", "version", "contactId", "key", "value"], readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .for(connection);

const readonlyOptions = (connection: IConnection) =>
  new CRUDRepositoryBuilder<ContactSettings>()
    .add({ entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("sqlite", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("mysql", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .add("postgres", { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), readProperties: ["id", "uuid", "created", "updated", "version", "contactId", "key", "value"] })
    .for(connection);

export class ContactSettingsRepository extends CRUDRepository<ContactSettings> {
  constructor(connection: IConnection) {
    super(connection, options(connection));
    this.createZeros = { key: "", value: "" };
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactSettings>(connection, readonlyOptions(connection));
}
