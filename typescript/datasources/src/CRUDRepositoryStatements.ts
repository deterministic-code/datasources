import type { Field } from "./builders/Field.ts";
import type { IDeleteStatement } from "./statements/IDeleteStatement.ts";
import type { IInsertStatement } from "./statements/IInsertStatement.ts";
import type { IUpdateStatement } from "./statements/IUpdateStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { SelectInsertedIdStatement } from "./statements/SelectInsertedIdStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";

export class CRUDRepositoryStatements extends ReadonlyRepositoryStatements {
  insert = (fields: Field[]): IInsertStatement => new InsertStatement(this.table, fields);

  update = (fields: Field[], where?: IWhere): IUpdateStatement =>
    new UpdateStatement(this.table, fields, where);

  delete = (where?: IWhere): IDeleteStatement => new DeleteStatement(this.table, where);

  selectInsertedId = (column: string): SelectInsertedIdStatement =>
    new SelectInsertedIdStatement(this.table, column);
}
