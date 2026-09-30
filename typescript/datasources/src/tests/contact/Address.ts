import { AddressType } from "./AddressType.ts";
import { Base } from "./Base.ts";
import { Contact } from "./Contact.ts";

export class Address extends Base {
  Contact = new Contact();
  AddressType = new AddressType();
  Line1 = "";
  Line2: string | null = null;
  City = "";
  Region: string | null = null;
  PostalCode: string | null = null;
  Country: string | null = null;
}
