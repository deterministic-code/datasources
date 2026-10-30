// schema-version: 1.0
/** Type ContactPreferences. */
export class ContactPreferences {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  key!: string;
  locale!: string;
  timezone!: string;
}
