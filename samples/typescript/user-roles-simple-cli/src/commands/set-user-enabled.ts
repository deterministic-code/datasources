import { ExecuteStoredProcedureStatement, Operator, UpdateStatement, WhereClause, execute, type Parameter } from "@datasources/core";
import type { App } from "../app.ts";
import { flag, ident } from "../schema.ts";
import { lookupId } from "./lookup-id.ts";

export const setUserEnabled = async (app: App, username: string, enabled: boolean): Promise<string> => {
  const provider = app.command.connection.provider;
  const updated = app.token();
  if (provider === "mysql") {
    const fields: Parameter[] = [
      { name: "p_username", value: username },
      { name: "p_is_enabled", value: flag(provider, enabled) },
      { name: "p_updated", value: app.bytes(updated) },
    ];
    await execute(app.command, new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
    return "";
  }
  const id = await lookupId(app.command, ident(provider, "users"), "username", username);
  const fields: Parameter[] = [
    { name: "is_enabled", value: flag(provider, enabled) },
    { name: "updated", value: app.bytes(updated) },
  ];
  const where = new WhereClause(new Operator.Equals("id", id));
  await execute(app.command, new UpdateStatement(ident(provider, "users"), fields, where), [...fields, ...where.parameters]);
  return "";
};
