// schema-version: 1.0
/** Type ContactAttribute. */
export const contactAttribute = (init: Partial<ContactAttribute>): ContactAttribute => ({ ...init } as ContactAttribute);
export class ContactAttribute {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  key!: string;
  value!: string;
}
