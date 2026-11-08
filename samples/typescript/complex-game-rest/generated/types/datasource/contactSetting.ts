// schema-version: 1.0
/** Type ContactSetting. */
export const contactSetting = (init: Partial<ContactSetting>): ContactSetting => ({ ...init } as ContactSetting);
export class ContactSetting {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  key!: string;
  value!: string;
}
