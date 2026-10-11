import type { Parameter } from "../Parameter.ts";
import type { IBulkUpdateStatement } from "./IBulkUpdateStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { flattenBulkParameters } from "./BulkInsertStatement.ts";

export type BulkUpdatedRow = { fields: Parameter[]; ids: Parameter[] };

export class BulkUpdateStatement implements IBulkUpdateStatement {
  constructor(
    readonly table: string,
    readonly set: readonly string[],
    readonly ids: readonly string[],
    readonly rows: readonly BulkUpdatedRow[],
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  get parameters(): Parameter[] {
    return flattenBulkParameters(
      [...this.set, ...this.ids],
      this.rows.map((row) => [...row.fields, ...row.ids]),
    );
  }
}
