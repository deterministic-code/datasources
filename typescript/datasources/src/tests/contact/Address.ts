import { Base } from "./Base.ts";

export class Address extends Base {
  contact_id = 0;
  line1 = "";
  line2: string | null = null;
  city = "";
  region: string | null = null;
  postal_code: string | null = null;
  country: string | null = null;
}
