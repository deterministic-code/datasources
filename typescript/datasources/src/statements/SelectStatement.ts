import type { ISelectStatement } from "./ISelectStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { IWhere } from "./IWhere.ts";
import type { Join } from "./Join.ts";
import type { OrderBy } from "./OrderBy.ts";
import type { SelectStatementOptions } from "./SelectStatementOptions.ts";

export class SelectStatement implements ISelectStatement {
  readonly where?: IWhere;
  readonly distinct: boolean;
  readonly table?: string;
  readonly fields: string[];
  readonly aliases: (string | undefined)[];
  readonly joins: Join[];
  readonly groupBy: string[];
  readonly having?: IWhere;
  readonly orderBy: OrderBy[];
  readonly limit?: number;
  readonly offset?: number;

  constructor(options: SelectStatementOptions) {
    this.where = options.where;
    this.distinct = options.distinct ?? false;
    this.table = options.table;
    this.fields = options.fields ?? [];
    this.aliases = options.aliases ?? [];
    this.joins = options.joins ?? [];
    this.groupBy = options.groupBy ?? [];
    this.having = options.having;
    this.orderBy = options.orderBy ?? [];
    this.limit = options.limit;
    this.offset = options.offset;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
