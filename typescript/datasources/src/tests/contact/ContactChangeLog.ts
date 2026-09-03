import { Base } from "./Base.ts";

export class ContactChangeLog extends Base {
  occurred_at = new Date(0);
  contact_id = 0;
  changed_by = "";
  change_summary = "";
}
