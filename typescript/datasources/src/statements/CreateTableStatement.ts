import type { Column } from "./Column.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";

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
}
