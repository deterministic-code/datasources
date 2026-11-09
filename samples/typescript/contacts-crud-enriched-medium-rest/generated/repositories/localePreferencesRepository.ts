import { LocalePreference } from "../types/view/localePreference";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, Schema, type IConnection } from "@deterministic-code/datasources";
export class LocalePreferencesRepository extends CRUDRepository<LocalePreference> {
  constructor(connection: IConnection) {
    super(connection, { type: LocalePreference, entity: "LocalePreference", entityMapper: new DictionaryMapper([["LocalePreference", "locale_preferences"]]), propertyMappings: new DictionaryMapper([]), ids: [], createProperties: ["locale", "timezone"], updateProperties: ["locale", "timezone"], readProperties: ["locale", "timezone"], schema: new Schema({ locale: { type: "string", nullable: false, hasDefault: false }, timezone: { type: "string", nullable: false, hasDefault: false } }) });
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LocalePreference>(connection, { type: LocalePreference, entity: "LocalePreference", entityMapper: new DictionaryMapper([["LocalePreference", "locale_preferences"]]), propertyMappings: new DictionaryMapper([]), readProperties: ["locale", "timezone"], schema: new Schema({ locale: { type: "string", nullable: false, hasDefault: false }, timezone: { type: "string", nullable: false, hasDefault: false } }) });
}
