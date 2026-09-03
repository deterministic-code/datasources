import { Contact } from "./Contact.ts";
import { ContactGroupBase } from "./ContactGroupBase.ts";

export class ContactGroup extends ContactGroupBase {
  members: Contact[] = [];
}
