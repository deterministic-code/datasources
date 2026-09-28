import {
  CreateTableStatement,
  CreateTableStatementRenderer,
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
    return statement instanceof CreateTableStatement && statement.table != null
      ? new CreateTableStatement(this.render(statement))
      : statement;
  }
}
