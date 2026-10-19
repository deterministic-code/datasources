import type { TypeMap } from "@deterministic-code/datasources";
import type { UserView } from "./user-view.ts";

export const usersView: TypeMap<UserView> = {
  key: "id",
  fields: {
    id: "id",
    username: "username",
    is_enabled: "is_enabled",
    created: "created",
    roles: {
      key: "role_id",
      fields: {
        id: "role_id",
        name: "role_name",
        is_enabled: "role_is_enabled",
      },
    },
    groups: {
      key: "group_id",
      fields: {
        id: "group_id",
        name: "group_name",
        is_enabled: "group_is_enabled",
      },
    },
  },
};
