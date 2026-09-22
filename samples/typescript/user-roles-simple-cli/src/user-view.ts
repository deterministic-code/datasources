import type { GroupView } from "./group-view.ts";
import type { RoleView } from "./role-view.ts";

export type UserView = {
  id: number;
  username: string;
  is_enabled: boolean;
  created: string;
  roles: RoleView[];
  groups: GroupView[];
};
