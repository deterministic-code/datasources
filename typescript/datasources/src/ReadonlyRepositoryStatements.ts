import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";

export class ReadonlyRepositoryStatements {
  constructor(
    protected readonly table: string,
    readonly readColumns: readonly string[],
  ) {}

  select = (where?: IWhere): ISelectStatement =>
    new SelectStatement({ table: this.table, fields: [...this.readColumns], where });
}
