import type { SpecField } from "../converters/SpecField.ts";
import type { ISqlSeedStatement } from "./ISqlSeedStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export type SqlSeedColumn = {
  column: string;
  field: SpecField;
  value: unknown;
};

export class SqlSeedStatement implements ISqlSeedStatement {
  constructor(
    readonly table: string,
    readonly columns: readonly SqlSeedColumn[],
    readonly kind: "insert" | "delete" = "insert",
    readonly identity = false,
    readonly idColumn?: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
