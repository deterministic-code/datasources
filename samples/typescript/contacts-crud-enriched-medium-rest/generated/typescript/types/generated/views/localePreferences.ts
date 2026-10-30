// schema-version: 1.0
/** View LocalePreferences. */
export const localePreferences = (init: Partial<LocalePreferences>): LocalePreferences => ({ ...init } as LocalePreferences);
export interface LocalePreferences {
  locale: string;
  timezone: string;
}
