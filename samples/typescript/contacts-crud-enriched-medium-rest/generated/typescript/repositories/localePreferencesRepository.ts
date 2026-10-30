import type { LocalePreferences } from "../types/generated/views/localePreferences";
import { CRUDRepository, DictionaryMapper, ReadonlyRepository, type IConnection } from "@deterministic-code/datasources";

export class LocalePreferencesRepository extends CRUDRepository<LocalePreferences> {
  constructor(connection: IConnection) {
    super(connection, { entity: "LocalePreferences", entityMapper: new DictionaryMapper([["LocalePreferences", "locale_preferences"]]), ids: [], createProperties: ["locale", "timezone"], updateProperties: ["locale", "timezone"], readProperties: ["locale", "timezone"] });
    this.createZeros = { locale: "", timezone: "" };
  }
  static createReadonly = (connection: IConnection) => new ReadonlyRepository<LocalePreferences>(connection, { entity: "LocalePreferences", entityMapper: new DictionaryMapper([["LocalePreferences", "locale_preferences"]]), readProperties: ["locale", "timezone"] });
}
