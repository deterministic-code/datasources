import type { Join } from "./Join.ts";
import type { IWhere } from "./IWhere.ts";
import type { OrderBy } from "./OrderBy.ts";

export type QueryStatementOptions = {
  table: string;
  fields: string[];
  where?: IWhere;
  joins?: Join[];
  groupBy?: string[];
  orderBy?: OrderBy[];
  limit?: number;
};
