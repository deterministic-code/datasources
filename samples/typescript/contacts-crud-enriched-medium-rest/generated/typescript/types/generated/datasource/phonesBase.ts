// schema-version: 1.0
import type { Base } from "./base";
/** Type PhonesBase. */
export interface PhonesBase extends Base {
  contact_id: string;
  number: string;
  label: string | null;
}
