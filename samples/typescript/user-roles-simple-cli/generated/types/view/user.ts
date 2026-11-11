// schema-version: 1.0
import type { Group } from "../datasource/group";
import type { Role } from "../datasource/role";
/** View User. */
export const user = (init: Partial<User>): User => Object.assign(new User(), init);
export class User {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: string = Buffer.from([0]).toString("base64");
  username: string = "";
  password: string = "";
  isEnabled: boolean = true;
  roles: Role[] = [];
  groups: Group[] = [];
}
