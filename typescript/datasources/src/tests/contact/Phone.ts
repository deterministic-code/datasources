import { Base } from "./Base.ts";
import { Contact } from "./Contact.ts";
import { PhoneType } from "./PhoneType.ts";

export class Phone extends Base {
  Contact = new Contact();
  PhoneType = new PhoneType();
  Number = "";
}
