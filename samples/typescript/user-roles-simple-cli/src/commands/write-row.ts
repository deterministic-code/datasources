import { ExecuteStoredProcedureStatement, InsertStatement, execute, type ICommand, type Parameter } from "@deterministic-code/datasources";

export const writeRow = (command: ICommand, table: string, procedure: string, insert: Parameter[], call: Parameter[], procedures: boolean) =>
  procedures && command.connection.provider === "mysql"
    ? execute(command, new ExecuteStoredProcedureStatement(procedure, call), call)
    : execute(command, new InsertStatement(table, insert), insert);
