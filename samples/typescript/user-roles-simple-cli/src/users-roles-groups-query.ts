import { Join, OrderBy, SelectStatement, type ISelectStatement } from "@deterministic-code/datasources";

export const usersRolesGroupsQuery = (): ISelectStatement =>
  new SelectStatement({
    table: "users",
    fields: [
      "users.id AS id",
      "users.username AS username",
      "users.is_enabled AS is_enabled",
      "users.created AS created",
      "roles.id AS role_id",
      "roles.name AS role_name",
      "roles.is_enabled AS role_is_enabled",
      "groups.id AS group_id",
      "groups.ugly_name AS group_name",
      "groups.is_enabled AS group_is_enabled",
    ],
    joins: [
      Join.left("user_roles", "users.id", "user_roles.user_id"),
      Join.left("roles", "user_roles.bad_role_id", "roles.id"),
      Join.left("user_groups", "users.id", "user_groups.user_id"),
      Join.left("groups", "user_groups.group_id", "groups.id"),
    ],
    orderBy: [OrderBy.asc("users.id"), OrderBy.asc("roles.id"), OrderBy.asc("groups.id")],
  });
