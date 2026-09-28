import type { Column } from "./Column.ts";
import { CreateTableStatementRenderer } from "./CreateTableStatementRenderer.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";

export class CreateTableStatement implements ICreateTableStatement {
  readonly table?: string;
  readonly columns: Column[];
  readonly constraints: ITableConstraint[];
  private readonly text?: string;

  constructor(text: string);
  constructor(table: string, columns: Column[], constraints?: ITableConstraint[]);
  constructor(tableOrText: string, columns?: Column[], constraints?: ITableConstraint[]) {
    if (columns) {
      this.table = tableOrText;
      this.columns = columns;
      this.constraints = constraints ?? [];
      return;
    }
    this.text = tableOrText;
    this.columns = [];
    this.constraints = [];
  }

  getText = (): string => this.text ?? new CreateTableStatementRenderer().render(this);
}
