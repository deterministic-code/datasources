// schema-version: 1.0
/** Type EmailType. */
export const emailType = (init: Partial<EmailType>): EmailType => Object.assign(new EmailType(), init);
export class EmailType {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  description: string = "";
}
