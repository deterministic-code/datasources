// schema-version: 1.0
import type { Base } from "./base";
/** Type AddressesBase. */
export interface AddressesBase extends Base {
  contact_id: string;
  line1: string;
  line2: string | null;
  city: string;
  region: string | null;
  postal_code: string | null;
  country: string | null;
}
