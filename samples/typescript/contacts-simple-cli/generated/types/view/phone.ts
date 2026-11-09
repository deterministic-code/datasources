// schema-version: 1.0
import type { Label } from "../datasource/label";
/** View Phone. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const phone = (init: Partial<Phone>): Phone => Object.assign(new Phone(), init);
export class Phone {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  number: string = "";
  displayName: string = "";
  extention: string = "";
  isPrimary: boolean = false;
  isVerified: boolean = false;
  phoneTypeName: string = "";
  description: string = "";
  attributes: Dictionary<string, string> = {};
  labels: Label[] = [];
}
