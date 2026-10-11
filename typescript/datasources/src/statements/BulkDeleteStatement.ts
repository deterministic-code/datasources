import type { Parameter } from "../Parameter.ts";
import type { IBulkDeleteStatement } from "./IBulkDeleteStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { flattenBulkParameters } from "./BulkInsertStatement.ts";

export class BulkDeleteStatement implements IBulkDeleteStatement {
  constructor(
    readonly table: string,
    readonly ids: readonly string[],
    readonly rows: readonly Parameter[][],
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  get parameters(): Parameter[] {
    return flattenBulkParameters(this.ids, this.rows);
  }
}
