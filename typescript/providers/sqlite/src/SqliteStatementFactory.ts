import {
  CreateTableStatement,
  InsertedIdStatement,
  SelectStatement,
  type IStatement,
} from "@datasources/core";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";

export class SqliteStatementFactory {
  transform(statement: IStatement): IStatement {
    if (statement instanceof InsertedIdStatement)
      return new SelectStatement(InsertedIdStatement.select("sqlite", statement.column));
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(new SqliteCreateTableStatementRenderer().render(statement))
      : statement;
  }
}
