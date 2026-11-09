// schema-version: 1.0
/** Type UserSetting. */
export const userSetting = (init: Partial<UserSetting>): UserSetting => Object.assign(new UserSetting(), init);
export class UserSetting {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  userId!: number;
  name: string = "";
  value: string = "";
}
