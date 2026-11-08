// schema-version: 1.0
/** Type LegacyContact. */
export const legacyContact = (init: Partial<LegacyContact>): LegacyContact => ({ ...init } as LegacyContact);
export interface LegacyContact {
  key: string;
  firstName: string;
  lastName: string;
  email: string | null;
  importedAt: Date | null;
}
