// schema-version: 1.0
/** Type PhoneBase. */
export interface PhoneBase {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  contactId: number;
  phoneTypeId: number;
  number: string;
  label: string | null;
}
