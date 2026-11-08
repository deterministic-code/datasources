// schema-version: 1.0
import type { AddressBase } from "../datasource/addressBase";
/** View Address. */
export const address = (init: Partial<Address>): Address => ({ ...init } as Address);
export interface Address extends Omit<AddressBase, "addressTypeId"> {
  addressTypeName: string;
  description: string | null;
}
