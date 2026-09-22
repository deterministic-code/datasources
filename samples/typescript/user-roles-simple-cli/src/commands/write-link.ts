import type { Parameter } from "@datasources/core";
import type { App } from "../app.ts";
import { ident } from "../schema.ts";
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
  const provider = app.command.connection.provider;
  const userId = await lookupId(app.command, ident(provider, "users"), "username", username);
  const rightId = await lookupId(app.command, ident(provider, rightTable), rightColumn, rightValue);
  const created = app.stamp();
  const updated = app.token();
  const insert: Parameter[] = [
    { name: "user_id", value: userId },
    { name: linkColumn, value: rightId },
    { name: "created", value: created },
    { name: "updated", value: app.bytes(updated) },
  ];
  const call: Parameter[] = [
    { name: "p_user_id", value: userId },
    { name: linkColumn === "bad_role_id" ? "p_role_id" : "p_group_id", value: rightId },
    { name: "p_created", value: created },
    { name: "p_updated", value: app.bytes(updated) },
  ];
  await writeRow(app.command, ident(provider, linkTable), procedure, insert, call);
  return "";
};
