// schema-version: 1.0
/** Type PhoneLabel. */
export const phoneLabel = (init: Partial<PhoneLabel>): PhoneLabel => ({ ...init } as PhoneLabel);
export class PhoneLabel {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  phoneId!: number;
  labelId!: number;
}
