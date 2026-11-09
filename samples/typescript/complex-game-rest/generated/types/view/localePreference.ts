// schema-version: 1.0
/** View LocalePreference. */
export const localePreference = (init: Partial<LocalePreference>): LocalePreference => Object.assign(new LocalePreference(), init);
export class LocalePreference {
  locale: string = "";
  timezone: string = "";
}
