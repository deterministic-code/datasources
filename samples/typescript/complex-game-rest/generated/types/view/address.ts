// schema-version: 1.0
/** View Address. */
export const address = (init: Partial<Address>): Address => Object.assign(new Address(), init);
export class Address {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  latitude!: number | null;
  labelNote!: string | null;
  floor!: number | null;
  score!: string | null;
  externalId!: string | null;
  isVerified!: boolean | null;
  verifiedAt!: Date | null;
  stamp!: string | null;
  contactId!: number;
  line1: string = "";
  line2!: string | null;
  city: string = "";
  region!: string | null;
  postalCode!: string | null;
  country!: string | null;
  addressTypeName: string = "";
  description!: string | null;
}
