import type { Column } from "./Column.ts";
import type { IAlterTableStatement } from "./IAlterTableStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";

export type AlterAction =
  | { kind: "addColumn"; column: Column }
  | { kind: "dropColumn"; name: string }
  | { kind: "alterColumn"; column: Column }
  | { kind: "renameColumn"; from: string; to: string }
  | { kind: "renameTable"; to: string }
  | { kind: "addConstraint"; constraint: ITableConstraint }
  | { kind: "dropConstraint"; name: string };

export class AlterTableStatement implements IAlterTableStatement {
  constructor(
    readonly table: string,
    readonly action: AlterAction,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
