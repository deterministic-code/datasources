import {
  CreateTableStatement,
  SelectInsertedIdStatement,
  SelectStatement,
  type IStatement,
} from "@datasources/core";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";

export class MysqlStatementFactory {
  transform(statement: IStatement): IStatement {
    if (statement instanceof SelectInsertedIdStatement)
      return new SelectStatement(`SELECT LAST_INSERT_ID() AS ${statement.column}`);
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(new MysqlCreateTableStatementRenderer().render(statement))
      : statement;
  }
}
