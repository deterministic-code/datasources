// schema-version: 1.0
import type { Contact } from "./contact";
/** View ContactGroup. */
export const contactGroup = (init: Partial<ContactGroup>): ContactGroup => Object.assign(new ContactGroup(), init);
export class ContactGroup {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description!: string | null;
  members: Contact[] = [];
}
