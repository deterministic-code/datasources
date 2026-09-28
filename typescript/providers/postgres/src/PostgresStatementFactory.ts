import {
  CreateTableStatement,
  InsertedIdStatement,
  SelectStatement,
  type IStatement,
} from "@datasources/core";
import { PostgresCreateTableStatementRenderer } from "./PostgresCreateTableStatementRenderer.ts";

export class PostgresStatementFactory {
  transform(statement: IStatement): IStatement {
    if (statement instanceof InsertedIdStatement)
      return new SelectStatement(InsertedIdStatement.select("postgres", statement.column));
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(new PostgresCreateTableStatementRenderer().render(statement))
      : statement;
  }
}
