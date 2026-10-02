import type { IStatementVisitor } from "./IStatementVisitor.ts";

export interface IStatement {
  accept(visitor: IStatementVisitor): string;
}
