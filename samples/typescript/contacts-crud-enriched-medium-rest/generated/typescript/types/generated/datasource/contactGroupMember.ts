// schema-version: 1.0
/** Type ContactGroupMember. */
export const contactGroupMember = (init: Partial<ContactGroupMember>): ContactGroupMember => ({ ...init } as ContactGroupMember);
export class ContactGroupMember {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  contactId!: number;
  contactGroupId!: number;
}
