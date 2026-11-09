// schema-version: 1.0
import type { Label } from "../datasource/label";
import type { Address } from "./address";
import type { Email } from "./email";
import type { Phone } from "./phone";
/** View Contact. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const contact = (init: Partial<Contact>): Contact => Object.assign(new Contact(), init);
export class Contact {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name: string = "";
  givenName: string = "";
  additionalName: string = "";
  pronunciation: string = "";
  familyName: string = "";
  nickName: string = "";
  jobTitle: string = "";
  organization: string = "";
  isStarred: boolean = false;
  isFavorite: boolean = false;
  isDeleted: boolean = false;
  genderTypeName: string = "";
  addresses: Address[] = [];
  emails: Email[] = [];
  phones: Phone[] = [];
  attributes: Dictionary<string, string> = {};
  labels: Label[] = [];
}
