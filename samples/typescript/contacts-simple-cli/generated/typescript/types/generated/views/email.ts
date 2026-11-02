// schema-version: 1.0
import type { EmailBase } from "../datasource/emailBase";
import type { Label } from "../datasource/label";
/** View Email. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const email = (init: Partial<Email>): Email => ({ ...init } as Email);
export interface Email extends Omit<EmailBase, "emailTypeId"> {
  emailTypeName: string;
  description: string;
  attributes: Dictionary<string, string>;
  labels: Label[];
}
