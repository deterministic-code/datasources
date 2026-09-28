import {
  CreateTableStatement,
  InsertedIdStatement,
  SelectStatement,
  type IStatement,
} from "@datasources/core";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";

export class MysqlStatementFactory {
  transform(statement: IStatement): IStatement {
    if (statement instanceof InsertedIdStatement)
      return new SelectStatement(InsertedIdStatement.select("mysql", statement.column));
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(new MysqlCreateTableStatementRenderer().render(statement))
      : statement;
  }
}
