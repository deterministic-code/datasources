import type { Join } from "./Join.ts";
import type { IWhere } from "./IWhere.ts";
import type { OrderBy } from "./OrderBy.ts";

export type SelectStatementOptions = {
  table?: string;
  fields?: string[];
  aliases?: (string | undefined)[];
  distinct?: boolean;
  where?: IWhere;
  joins?: Join[];
  groupBy?: string[];
  having?: IWhere;
  orderBy?: OrderBy[];
  limit?: number;
  offset?: number;
};
