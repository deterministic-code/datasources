// schema-version: 1.0
/** Type ContactBase. */
export const contactBase = (init: Partial<ContactBase>): ContactBase => Object.assign(new ContactBase(), init);
export class ContactBase {
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
  genderTypeId!: number;
  organization: string = "";
  isStarred: boolean = false;
  isFavorite: boolean = false;
  isDeleted: boolean = false;
}
