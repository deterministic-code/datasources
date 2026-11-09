// schema-version: 1.0
/** Type UserBase. */
export const userBase = (init: Partial<UserBase>): UserBase => Object.assign(new UserBase(), init);
export class UserBase {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  username: string = "";
  password: string = "";
  isEnabled: boolean = true;
}
