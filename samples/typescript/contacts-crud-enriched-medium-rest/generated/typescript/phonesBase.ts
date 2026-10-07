// schema-version: 1.0
import type { Base } from "./base";
/** Type PhonesBase. */
export interface PhonesBase extends Base {
  contactId: string;
  number: string;
  label: string | null;
}
