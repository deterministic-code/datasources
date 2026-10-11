import { statementParameters } from "./statementParameters.ts";

export const statementUsesParameter = (sql: string, name: string): boolean =>
  statementParameters(sql).includes(name.replace(/^[$:@]/, ""));
