// schema-version: 1.0
/** View LegacyContact. */
export const legacyContact = (init: Partial<LegacyContact>): LegacyContact => Object.assign(new LegacyContact(), init);
export class LegacyContact {
  key: string = "";
  firstName: string = "";
  lastName: string = "";
  email!: string | null;
  importedAt!: Date | null;
}
