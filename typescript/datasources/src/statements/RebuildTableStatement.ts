import type { CreateTableStatement } from "./CreateTableStatement.ts";
import type { IRebuildTableStatement } from "./IRebuildTableStatement.ts";
import type { IStatement } from "./IStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { RebuildColumn } from "./RebuildColumn.ts";

export class RebuildTableStatement implements IRebuildTableStatement {
  readonly recreate: IStatement[];

  constructor(
    readonly table: string,
    readonly create: CreateTableStatement,
    readonly columns: readonly RebuildColumn[],
    recreate: readonly IStatement[] = [],
  ) {
    this.recreate = [...recreate];
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
