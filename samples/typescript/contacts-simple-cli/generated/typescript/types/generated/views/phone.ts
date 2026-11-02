// schema-version: 1.0
import type { Label } from "../datasource/label";
import type { PhoneBase } from "../datasource/phoneBase";
/** View Phone. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const phone = (init: Partial<Phone>): Phone => ({ ...init } as Phone);
export interface Phone extends Omit<PhoneBase, "phoneTypeId"> {
  phoneTypeName: string;
  description: string;
  attributes: Dictionary<string, string>;
  labels: Label[];
}
