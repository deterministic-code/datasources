import type { IStatement } from "./IStatement.ts";
import type { IWhere } from "./IWhere.ts";

export interface ISelectStatement extends IStatement {
  readonly where?: IWhere;
}
