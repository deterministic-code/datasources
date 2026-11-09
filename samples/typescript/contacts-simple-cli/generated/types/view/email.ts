// schema-version: 1.0
import type { Label } from "../datasource/label";
/** View Email. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const email = (init: Partial<Email>): Email => Object.assign(new Email(), init);
export class Email {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  address: string = "";
  displayName: string = "";
  isPrimary: boolean = false;
  emailTypeName: string = "";
  description: string = "";
  attributes: Dictionary<string, string> = {};
  labels: Label[] = [];
}
