// schema-version: 1.0
/** Type PhoneBase. */
export const phoneBase = (init: Partial<PhoneBase>): PhoneBase => ({ ...init } as PhoneBase);
export class PhoneBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  phoneTypeId!: number;
  contactId!: number;
  number!: string;
  displayName!: string;
  extention!: string;
  isPrimary!: boolean;
  isVerified!: boolean;
}
