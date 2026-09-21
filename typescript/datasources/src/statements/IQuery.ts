import type { IStatement } from "./IStatement.ts";
import type { IWhere } from "./IWhere.ts";

export interface IQuery extends IStatement {
  readonly where?: IWhere;
}
