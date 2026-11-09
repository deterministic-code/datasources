// schema-version: 1.0
/** Type AddressBase. */
export const addressBase = (init: Partial<AddressBase>): AddressBase => Object.assign(new AddressBase(), init);
export class AddressBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  addressTypeId!: number;
  displayName: string = "";
  line1: string = "";
  line2: string = "";
  line3: string = "";
  neighborhood: string = "";
  district: string = "";
  city: string = "";
  stateProvince: string = "";
  postalCode: string = "";
  countryCode: string = "";
  formattedAddress: string = "";
  latitude!: number | null;
  longitude!: number | null;
  isPrimary: boolean = false;
}
