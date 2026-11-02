// schema-version: 1.0
/** Type ContactGroupsBase. */
export const contactGroupsBase = (init: Partial<ContactGroupsBase>): ContactGroupsBase => ({ ...init } as ContactGroupsBase);
export class ContactGroupsBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  name!: string;
  description: string | null;
}
