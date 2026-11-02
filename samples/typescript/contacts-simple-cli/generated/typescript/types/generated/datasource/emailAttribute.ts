// schema-version: 1.0
/** Type EmailAttribute. */
export const emailAttribute = (init: Partial<EmailAttribute>): EmailAttribute => ({ ...init } as EmailAttribute);
export class EmailAttribute {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  emailId!: number;
  key!: string;
  value!: string;
}
