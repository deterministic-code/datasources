import {
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectStatement,
  type IStatement,
} from "@datasources/core";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";

export class SqliteStatementFactory {
  transform(statement: IStatement): IStatement {
    if (statement instanceof SelectInsertedIdStatement)
      return new SelectStatement(`SELECT last_insert_rowid() AS ${statement.column}`);
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(new SqliteCreateTableStatementRenderer().render(statement))
      : statement;
  }
}
