import { Address } from "./Address.ts";
import { ContactBase } from "./ContactBase.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { Phone } from "./Phone.ts";

export class Contact extends ContactBase {
  ContactSource = new ContactSource();
  ContactType = new ContactType();
  Addresses: Address[] = [];
  Phones: Phone[] = [];
}
