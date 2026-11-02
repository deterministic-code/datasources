import type { LocalePreferences } from "../types/generated/views/localePreferences";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class LocalePreferencesRepository extends CRUDRepository<LocalePreferences> {
  constructor(connection: IConnection) {
    super(connection, { entity: "LocalePreferences", entityMapper: new DictionaryMapper([["LocalePreferences", "locale_preferences"]]), propertyMappings: new DictionaryMapper([]), ids: [], createProperties: ["locale", "timezone"], updateProperties: ["locale", "timezone"], readProperties: ["locale", "timezone"], schema: new Schema({ locale: { type: "string", nullable: false, hasDefault: false }, timezone: { type: "string", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LocalePreferences>(connection, { entity: "LocalePreferences", entityMapper: new DictionaryMapper([["LocalePreferences", "locale_preferences"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["locale", "timezone"], schema: new Schema({ locale: { type: "string", nullable: false, hasDefault: false }, timezone: { type: "string", nullable: false, hasDefault: false } }) });
}
