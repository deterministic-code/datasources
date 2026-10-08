// schema-version: 1.0
import type { PhoneBase } from "../datasource/phoneBase";
/** View Phone. */
export interface Phone extends PhoneBase {
  phoneTypeName: string;
  description: string | null;
}
