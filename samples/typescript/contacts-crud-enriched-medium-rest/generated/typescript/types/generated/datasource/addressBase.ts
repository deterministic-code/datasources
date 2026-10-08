// schema-version: 1.0
/** Type AddressBase. */
export interface AddressBase {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  contactId: number;
  addressTypeId: number;
  line1: string;
  line2: string | null;
  city: string;
  region: string | null;
  postalCode: string | null;
  country: string | null;
}
