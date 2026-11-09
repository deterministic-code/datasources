// schema-version: 1.0
/** View Phone. */
export const phone = (init: Partial<Phone>): Phone => Object.assign(new Phone(), init);
export class Phone {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  number: string = "";
  label!: string | null;
  phoneTypeName: string = "";
  description!: string | null;
}
