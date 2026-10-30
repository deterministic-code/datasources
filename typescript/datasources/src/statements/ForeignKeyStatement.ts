import { constraintNameOf } from "./ConstraintName.ts";
import type { IForeignKeyStatement } from "./IForeignKeyStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { ReferentialAction } from "./ForeignKey.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class ForeignKeyStatement implements IForeignKeyStatement {
  constructor(
    readonly table: string,
    readonly name: string,
    readonly columns: readonly string[],
    readonly referencesTable: string,
    readonly referencesColumns: readonly string[],
    readonly kind: "add" | "drop" = "add",
    readonly onDelete?: ReferentialAction,
    readonly onUpdate?: ReferentialAction,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  static nameOf = (entity: string, naming: StatementNaming, ...parts: string[]): string =>
    constraintNameOf(entity, naming, ...parts);
}
