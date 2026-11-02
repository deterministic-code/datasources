import type { Parameter } from "@deterministic-code/datasources";
import type { App } from "../app.ts";
import { lookupId } from "./lookup-id.ts";
import { writeRow } from "./write-row.ts";

export const writeLink = async (
  app: App,
  username: string,
  rightTable: string,
  rightColumn: string,
  rightValue: string,
  linkTable: string,
  linkColumn: string,
  procedure: string,
): Promise<string> => {
  const userId = await lookupId(app.command, "users", "username", username);
  const rightId = await lookupId(app.command, rightTable, rightColumn, rightValue);
  const insert: Parameter[] = [
    { name: "user_id", value: userId },
    { name: linkColumn, value: rightId },
  ];
  const call: Parameter[] = [
    { name: "p_user_id", value: userId },
    { name: linkColumn === "bad_role_id" ? "p_role_id" : "p_group_id", value: rightId },
  ];
  await writeRow(app.command, linkTable, procedure, insert, call, app.procedures);
  return "";
};
