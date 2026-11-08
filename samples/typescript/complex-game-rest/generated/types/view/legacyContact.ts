// schema-version: 1.0
import type { LegacyContact as LegacyContactBase } from "../datasource/legacyContact";
/** View LegacyContact. */
export const legacyContact = (init: Partial<LegacyContact>): LegacyContact => ({ ...init } as LegacyContact);
export interface LegacyContact extends LegacyContactBase {}
