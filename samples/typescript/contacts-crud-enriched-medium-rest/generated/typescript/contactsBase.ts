// schema-version: 1.0
import type { Base } from "./base";
/** Type ContactsBase. */
export interface ContactsBase extends Base {
  contact_source_id: string;
  first_name: string;
  last_name: string;
  email: string | null;
  notes: string | null;
}
