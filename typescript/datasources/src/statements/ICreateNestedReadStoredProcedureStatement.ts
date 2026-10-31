import type { NestedSelectStatement } from "./NestedSelectStatement.ts";
import type { IStatement } from "./IStatement.ts";
import type { NestedWhereArgument } from "./NestedWhereArgument.ts";

export interface ICreateNestedReadStoredProcedureStatement extends IStatement {
  readonly name: string;
  readonly nested: NestedSelectStatement;
  readonly arguments: readonly NestedWhereArgument[];
}
