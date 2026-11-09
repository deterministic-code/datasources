// schema-version: 1.0
/** Type UserRole. */
export const userRole = (init: Partial<UserRole>): UserRole => Object.assign(new UserRole(), init);
export class UserRole {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  userId!: number;
  roleId!: number;
}
