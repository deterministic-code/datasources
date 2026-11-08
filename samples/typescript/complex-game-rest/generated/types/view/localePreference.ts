// schema-version: 1.0
/** View LocalePreference. */
export const localePreference = (init: Partial<LocalePreference>): LocalePreference => ({ ...init } as LocalePreference);
export interface LocalePreference {
  locale: string;
  timezone: string;
}
