// schema-version: 1.0
/** Type UserGroup. */
export const userGroup = (init: Partial<UserGroup>): UserGroup => Object.assign(new UserGroup(), init);
export class UserGroup {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  userId!: number;
  groupId!: number;
}
