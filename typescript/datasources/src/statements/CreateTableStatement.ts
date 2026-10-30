import type { Column } from "./Column.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class CreateTableStatement implements ICreateTableStatement {
  readonly table: string;
  readonly columns: Column[];
  readonly constraints: ITableConstraint[];

  constructor(table: string, columns: Column[], constraints: ITableConstraint[] = []) {
    this.table = table;
    this.columns = columns;
    this.constraints = constraints;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  static tableName = (entity: string, naming: StatementNaming): string =>
    naming.isVerbatim(entity)
      ? entity
      : naming.letter(naming.convert(naming.physicalStem(entity), naming.tableNameCasing));

  static columnName = (field: string, naming: StatementNaming): string =>
    naming.isVerbatim(field) ? field : naming.convert(field, naming.fieldNameCasing);
}
