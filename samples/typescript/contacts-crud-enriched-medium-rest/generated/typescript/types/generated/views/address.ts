// schema-version: 1.0
import type { AddressBase } from "../datasource/addressBase";
/** View Address. */
export interface Address extends Omit<AddressBase, "addressTypeId"> {
  addressTypeName: string;
  description: string | null;
}
