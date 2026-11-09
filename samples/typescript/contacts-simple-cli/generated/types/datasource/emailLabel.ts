// schema-version: 1.0
/** Type EmailLabel. */
export const emailLabel = (init: Partial<EmailLabel>): EmailLabel => Object.assign(new EmailLabel(), init);
export class EmailLabel {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  emailId!: number;
  labelId!: number;
}
