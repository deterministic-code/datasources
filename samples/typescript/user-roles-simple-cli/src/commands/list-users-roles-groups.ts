import { FlatToNestedMapper, executeReader } from "@deterministic-code/datasources";
import type { App } from "../app.ts";
import { asUserView } from "../as-user-view.ts";
import { usersRolesGroupsQuery } from "../users-roles-groups-query.ts";
import { usersView } from "../users-view.ts";

export type ListUsersRolesGroups = { kind: "list-users-roles-groups" };

export const listUsersRolesGroups = async (app: App): Promise<string> => {
  const reader = await executeReader(app.command, usersRolesGroupsQuery(app.command.connection.provider));
  const rows = [];
  while (await reader.next()) if (reader.current) rows.push(reader.current);
  return JSON.stringify(asUserView(new FlatToNestedMapper(usersView).map(rows)));
};
