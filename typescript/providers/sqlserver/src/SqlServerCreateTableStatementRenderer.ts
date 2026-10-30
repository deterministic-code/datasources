import { CreateTableStatementTextRenderer, type Column } from "@deterministic-code/datasources";
import { SqlServerDriver } from "./SqlServerDriver.ts";

export class SqlServerCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected quote(name: string): string {
    return SqlServerDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return SqlServerDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${SqlServerDriver.nativeDefault(column)}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${SqlServerDriver.nativeDefault(column)}`;
  }

  protected identity(column: Column): string {
    return column.identity ? " IDENTITY(1,1)" : "";
  }
}
