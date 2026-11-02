// schema-version: 1.0
import type { ContactBase } from "../datasource/contactBase";
import type { Label } from "../datasource/label";
import type { Address } from "./address";
import type { Email } from "./email";
import type { Phone } from "./phone";
/** View Contact. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const contact = (init: Partial<Contact>): Contact => ({ ...init } as Contact);
export interface Contact extends Omit<ContactBase, "genderTypeId"> {
  genderTypeName: string;
  addresses: Address[];
  emails: Email[];
  phones: Phone[];
  attributes: Dictionary<string, string>;
  labels: Label[];
}
