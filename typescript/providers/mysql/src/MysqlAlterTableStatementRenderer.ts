import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
  type ITableConstraint,
  type NotNull,
} from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";
import { fitMysqlConstraint, fitMysqlIdentifier } from "./MysqlIdentifier.ts";

export class MysqlAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
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

  protected override constraint(constraint: ITableConstraint): string {
    return super.constraint(fitMysqlConstraint(constraint));
  }

  protected override actionSql(action: AlterAction): string {
    if (action.kind === "alterColumn") return `MODIFY COLUMN ${this.column(action.column)}`;
    if (action.kind === "dropConstraint") return `DROP CONSTRAINT ${this.quote(fitMysqlIdentifier(action.name))}`;
    return super.actionSql(action);
  }
}
