import { Base } from "./Base.ts";

export class ContactBase extends Base {
  contact_source_id = 0;
  contact_type_id = 0;
  first_name = "";
  last_name = "";
  email: string | null = null;
  notes: string | null = null;
}
