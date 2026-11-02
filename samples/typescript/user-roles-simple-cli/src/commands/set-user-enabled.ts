import { ExecuteStoredProcedureStatement, Operator, UpdateStatement, Where, execute, type Parameter } from "@deterministic-code/datasources";
import type { App } from "../app.ts";
import { lookupId } from "./lookup-id.ts";

export const setUserEnabled = async (app: App, username: string, enabled: boolean): Promise<string> => {
  const provider = app.command.connection.provider;
  const updated = app.token();
  if (app.procedures && provider === "mysql") {
    const fields: Parameter[] = [
      { name: "p_username", value: username },
      { name: "p_is_enabled", value: enabled },
      { name: "p_updated", value: app.bytes(updated) },
    ];
    await execute(app.command, new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
    return "";
  }
  const id = await lookupId(app.command, "users", "username", username);
  const fields: Parameter[] = [
    { name: "is_enabled", value: enabled },
    { name: "updated", value: app.bytes(updated) },
  ];
  const where = new Where(new Operator.Equals("id", id));
  await execute(app.command, new UpdateStatement("users", fields, where), [...fields, ...where.parameters]);
  return "";
};
