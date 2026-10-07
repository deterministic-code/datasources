// schema-version: 1.0
import type { Base } from "./base";
/** Type ContactsBase. */
export interface ContactsBase extends Base {
  contactSourceId: string;
  firstName: string;
  lastName: string;
  email: string | null;
  notes: string | null;
}
