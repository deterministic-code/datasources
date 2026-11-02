// schema-version: 1.0
/** Type ContactLabel. */
export const contactLabel = (init: Partial<ContactLabel>): ContactLabel => ({ ...init } as ContactLabel);
export class ContactLabel {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  labelId!: number;
}
