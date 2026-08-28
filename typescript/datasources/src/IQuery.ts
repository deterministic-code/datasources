import type { IStatement } from "./builders/IStatement.ts";
import type { IWhere } from "./IWhere.ts";

export interface IQuery extends IStatement {
  readonly where?: IWhere;
}
