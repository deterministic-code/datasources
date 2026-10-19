import { CreateTableStatementTextRenderer, type Column } from "@deterministic-code/datasources";

export class SqlServerCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected identity(column: Column): string {
    return column.identity ? " IDENTITY(1,1)" : "";
  }
}
