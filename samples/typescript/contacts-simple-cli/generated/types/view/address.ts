// schema-version: 1.0
import type { Label } from "../datasource/label";
/** View Address. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const address = (init: Partial<Address>): Address => Object.assign(new Address(), init);
export class Address {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
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
  addressTypeName: string = "";
  description: string = "";
  attributes: Dictionary<string, string> = {};
  labels: Label[] = [];
}
