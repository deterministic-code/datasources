import type { IIdentityInsertStatement } from "./IIdentityInsertStatement.ts";
import type { InsertStatement } from "./InsertStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class IdentityInsertStatement implements IIdentityInsertStatement {
  constructor(
    readonly table: string,
    readonly idColumn: string,
    readonly inserts: readonly InsertStatement[],
    readonly identity = false,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
