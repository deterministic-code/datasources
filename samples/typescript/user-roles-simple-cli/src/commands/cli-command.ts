import type { AddGroup } from "./add-group.ts";
import type { AddRole } from "./add-role.ts";
import type { AddUser } from "./add-user.ts";
import type { AddUserGroup } from "./add-user-group.ts";
import type { AddUserRole } from "./add-user-role.ts";
import type { DisableUser } from "./disable-user.ts";
import type { EnableUser } from "./enable-user.ts";
import type { ListUsersRolesGroups } from "./list-users-roles-groups.ts";

export type CliCommand =
  | AddUser
  | AddRole
  | AddGroup
  | AddUserRole
  | AddUserGroup
  | EnableUser
  | DisableUser
  | ListUsersRolesGroups;
