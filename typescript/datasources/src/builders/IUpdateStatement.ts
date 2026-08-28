import type { IStatement } from "./IStatement.ts";
import type { IWhere } from "../IWhere.ts";

export interface IUpdateStatement extends IStatement {
  readonly where?: IWhere;
}
