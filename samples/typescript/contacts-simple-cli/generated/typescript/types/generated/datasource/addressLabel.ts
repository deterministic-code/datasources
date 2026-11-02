// schema-version: 1.0
/** Type AddressLabel. */
export const addressLabel = (init: Partial<AddressLabel>): AddressLabel => ({ ...init } as AddressLabel);
export class AddressLabel {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  addressId!: number;
  labelId!: number;
}
