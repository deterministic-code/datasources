import type { ContactSettings } from "../types/generated/datasource/contactSettings.ts";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class ContactSettingsRepository extends CRUDRepository<ContactSettings> {
  constructor(connection: IConnection) {
    super(connection, { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: [], readProperties: ["contactId", "key", "value"] });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<ContactSettings>(connection, { entity: "ContactSettings", entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]), propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]), ids: [], readProperties: ["contactId", "key", "value"] });
}
