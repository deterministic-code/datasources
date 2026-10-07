// schema-version: 1.0
import type { Base } from "./base";
/** Type AddressesBase. */
export interface AddressesBase extends Base {
  contactId: string;
  line1: string;
  line2: string | null;
  city: string;
  region: string | null;
  postalCode: string | null;
  country: string | null;
}
