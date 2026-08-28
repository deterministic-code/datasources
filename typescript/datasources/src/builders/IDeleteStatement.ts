import type { IStatement } from "./IStatement.ts";
import type { IWhere } from "../IWhere.ts";

export interface IDeleteStatement extends IStatement {
  readonly where?: IWhere;
}
