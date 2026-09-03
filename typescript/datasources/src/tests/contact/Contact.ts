import { Address } from "./Address.ts";
import { ContactBase } from "./ContactBase.ts";
import { Phone } from "./Phone.ts";

export class Contact extends ContactBase {
  contact_source_name = "";
  contact_type_name = "";
  addresses: Address[] = [];
  phones: Phone[] = [];
}
