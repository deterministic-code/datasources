import { AlterTableStatementTextRenderer, type AlterAction, type Column } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected quote(name: string): string {
    return OracleDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return OracleDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${OracleDriver.nativeDefault(column)}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${OracleDriver.nativeDefault(column)}`;
  }
  protected override actionSql(action: AlterAction): string {
    return action.kind === "addColumn"
      ? `ADD (${this.column(action.column)})`
      : action.kind === "alterColumn"
        ? `MODIFY (${this.column(action.column)})`
        : super.actionSql(action);
  }
}
