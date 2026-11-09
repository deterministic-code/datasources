// schema-version: 1.0
import type { Group } from "../datasource/group";
import type { Role } from "../datasource/role";
/** View User. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const user = (init: Partial<User>): User => Object.assign(new User(), init);
export class User {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  username: string = "";
  password: string = "";
  isEnabled: boolean = true;
  settings: Dictionary<string, string> = {};
  roles: Role[] = [];
  groups: Group[] = [];
}
