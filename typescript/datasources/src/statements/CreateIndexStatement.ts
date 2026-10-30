import { constraintNameOf } from "./ConstraintName.ts";
import type { ICreateIndexStatement } from "./ICreateIndexStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class CreateIndexStatement implements ICreateIndexStatement {
  readonly name: string;
  readonly table: string;
  readonly columns: string[];
  readonly unique: boolean;

  constructor(name: string, table: string, columns: string[], unique = false) {
    this.name = name;
    this.table = table;
    this.columns = columns;
    this.unique = unique;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  static nameOf = (entity: string, naming: StatementNaming, ...parts: string[]): string =>
    constraintNameOf(entity, naming, ...parts);
}
