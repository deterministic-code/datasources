// schema-version: 1.0
/** Type AddressAttribute. */
export const addressAttribute = (init: Partial<AddressAttribute>): AddressAttribute => Object.assign(new AddressAttribute(), init);
export class AddressAttribute {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  addressId!: number;
  key: string = "";
  value: string = "";
}
