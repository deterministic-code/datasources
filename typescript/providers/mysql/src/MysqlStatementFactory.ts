import {
  CreateTableStatement,
  CreateTableStatementRenderer,
  InsertedIdStatement,
  SelectStatement,
  type Column,
  type IStatement,
} from "@datasources/core";
import { MysqlDriver } from "./MysqlDriver.ts";

const canonical = new Set(["int", "uuid", "string", "datetime", "bool", "decimal", "float", "binary"]);

export class MysqlStatementFactory extends CreateTableStatementRenderer {
  protected typeName(column: Column): string {
    return canonical.has(column.type) ? new MysqlDriver().columnType(column.type) : column.type;
  }

  protected identity(column: Column): string {
    return column.identity ? " AUTO_INCREMENT" : "";
  }

  transform(statement: IStatement): IStatement {
    if (statement instanceof InsertedIdStatement)
      return new SelectStatement(`SELECT LAST_INSERT_ID() AS ${statement.column}`);
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(this.render(statement))
      : statement;
  }
}
