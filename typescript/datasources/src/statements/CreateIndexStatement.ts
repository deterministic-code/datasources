import type { ICreateIndexStatement } from "./ICreateIndexStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

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
}
