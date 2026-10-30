// schema-version: 1.0
import type { PhoneBase } from "../datasource/phoneBase";
/** View Phone. */
export const phone = (init: Partial<Phone>): Phone => ({ ...init } as Phone);
export interface Phone extends Omit<PhoneBase, "phoneTypeId"> {
  phoneTypeName: string;
  description: string | null;
}
