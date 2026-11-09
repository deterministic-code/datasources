// schema-version: 1.0
/** Type EmailBase. */
export const emailBase = (init: Partial<EmailBase>): EmailBase => Object.assign(new EmailBase(), init);
export class EmailBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  emailTypeId!: number;
  contactId!: number;
  address: string = "";
  displayName: string = "";
  isPrimary: boolean = false;
}
