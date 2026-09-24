import { Join, QueryStatement, type IQuery } from "@datasources/core";
import { ident } from "./schema.ts";

export const usersRolesGroupsQuery = (provider = "sqlite"): IQuery => {
  const users = ident(provider, "users");
  const roles = ident(provider, "roles");
  const groups = ident(provider, "groups");
  const userRoles = ident(provider, "user_roles");
  const userGroups = ident(provider, "user_groups");
  return new QueryStatement({
    table: users,
    fields: [
      `${users}.id AS id`,
      `${users}.username AS username`,
      `${users}.is_enabled AS is_enabled`,
      `${users}.created AS created`,
      `${roles}.id AS role_id`,
      `${roles}.name AS role_name`,
      `${roles}.is_enabled AS role_is_enabled`,
      `${groups}.id AS group_id`,
      `${groups}.ugly_name AS group_name`,
      `${groups}.is_enabled AS group_is_enabled`,
    ],
    joins: [
      Join.left(userRoles, `${users}.id`, `${userRoles}.user_id`),
      Join.left(roles, `${userRoles}.bad_role_id`, `${roles}.id`),
      Join.left(userGroups, `${users}.id`, `${userGroups}.user_id`),
      Join.left(groups, `${userGroups}.group_id`, `${groups}.id`),
    ],
  });
};
