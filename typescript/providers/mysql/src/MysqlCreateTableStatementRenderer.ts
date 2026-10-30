import { CreateTableStatementTextRenderer, type Column, type NotNull } from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";

export class MysqlCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected quote(name: string): string {
    return MysqlDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return MysqlDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null ? "" : ` DEFAULT ${MysqlDriver.nativeDefault(column)}`;
  }

  protected notNullSql(value?: NotNull): string {
    return value == null ? "" : " NOT NULL";
  }

  protected identity(column: Column): string {
    return column.identity ? " AUTO_INCREMENT" : "";
  }
}
