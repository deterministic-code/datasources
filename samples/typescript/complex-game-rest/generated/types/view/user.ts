// schema-version: 1.0
import type { Group } from "../datasource/group";
import type { Role } from "../datasource/role";
import type { UserBase } from "../datasource/userBase";
/** View User. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const user = (init: Partial<User>): User => ({ ...init } as User);
export interface User extends UserBase {
  settings: Dictionary<string, string>;
  roles: Role[];
  groups: Group[];
}
