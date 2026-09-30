import { Base } from "./Base.ts";

export class ContactBase extends Base {
  FirstName = "";
  LastName = "";
  Email: string | null = null;
  Notes: string | null = null;
}
