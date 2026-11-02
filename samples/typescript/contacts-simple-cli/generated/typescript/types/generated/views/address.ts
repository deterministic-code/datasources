// schema-version: 1.0
import type { AddressBase } from "../datasource/addressBase";
import type { Label } from "../datasource/label";
/** View Address. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const address = (init: Partial<Address>): Address => ({ ...init } as Address);
export interface Address extends Omit<AddressBase, "addressTypeId"> {
  addressTypeName: string;
  description: string;
  attributes: Dictionary<string, string>;
  labels: Label[];
}
