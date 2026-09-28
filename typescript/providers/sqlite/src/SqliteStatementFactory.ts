import {
  CreateTableStatement,
  CreateTableStatementRenderer,
  InsertedIdStatement,
  SelectStatement,
  type Column,
  type IStatement,
} from "@datasources/core";
import { SqliteDriver } from "./SqliteDriver.ts";

const canonical = new Set(["int", "uuid", "string", "datetime", "bool", "decimal", "float", "binary"]);

export class SqliteStatementFactory extends CreateTableStatementRenderer {
  protected typeName(column: Column): string {
    return canonical.has(column.type) ? new SqliteDriver().columnType(column.type) : column.type;
  }

  transform(statement: IStatement): IStatement {
    if (statement instanceof InsertedIdStatement)
      return new SelectStatement(`SELECT last_insert_rowid() AS ${statement.column}`);
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(this.render(statement))
      : statement;
  }
}
