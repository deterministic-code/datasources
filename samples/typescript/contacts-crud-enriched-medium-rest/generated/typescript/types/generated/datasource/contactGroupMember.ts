// schema-version: 1.0
/** Type ContactGroupMember. */
export interface ContactGroupMember {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  contactId: string;
  contactGroupId: string;
}
