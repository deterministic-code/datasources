// schema-version: 1.0
/** Type ContactGroupsBase. */
export interface ContactGroupsBase {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  name: string;
  description: string | null;
}
