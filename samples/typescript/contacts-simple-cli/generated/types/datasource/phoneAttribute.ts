// schema-version: 1.0
/** Type PhoneAttribute. */
export const phoneAttribute = (init: Partial<PhoneAttribute>): PhoneAttribute => Object.assign(new PhoneAttribute(), init);
export class PhoneAttribute {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  phoneId!: number;
  key: string = "";
  value: string = "";
}
