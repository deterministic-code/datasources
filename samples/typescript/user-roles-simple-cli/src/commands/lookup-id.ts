import { Operator, QueryStatement, Where, executeReader, type ICommand } from "@datasources/core";

export const lookupId = async (command: ICommand, table: string, column: string, value: string): Promise<number> => {
  const where = new Where(new Operator.Equals(column, value));
  const reader = await executeReader(command, new QueryStatement({ table, fields: ["id"], where }), where.parameters);
  if (!(await reader.next()) || reader.current == null)
    throw new Error(`${table} '${value}' not found`);
  const id = reader.current.id;
  return typeof id === "number" ? id : (() => {
    throw new Error(`${table} '${value}' not found`);
  })();
};
