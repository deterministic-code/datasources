// schema-version: 1.0
/** Type PhoneBase. */
export class PhoneBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  phoneTypeId!: number;
  number!: string;
  label: string | null;
}
