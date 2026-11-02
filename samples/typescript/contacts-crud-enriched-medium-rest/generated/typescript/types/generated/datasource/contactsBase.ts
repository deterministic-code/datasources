// schema-version: 1.0
/** Type ContactsBase. */
export class ContactsBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactSourceId!: number;
  firstName!: string;
  lastName!: string;
  email: string | null;
  notes: string | null;
}
