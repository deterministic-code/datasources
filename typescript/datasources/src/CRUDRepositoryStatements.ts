import type { Cell } from "./converters/Cell.ts";
import type { Field } from "./builders/Field.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import { StatementMappers } from "./StatementMappers.ts";
import { DeleteStatement } from "./statements/DeleteStatement.ts";
import { InsertStatement } from "./statements/InsertStatement.ts";
import { SelectInsertedIdStatement } from "./statements/SelectInsertedIdStatement.ts";
import { UpdateStatement } from "./statements/UpdateStatement.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";

const same = (_name: string, value: Cell): Cell => value;

export class CRUDRepositoryStatements extends ReadonlyRepositoryStatements {
  private readonly mappings: StatementMappers;

  constructor(
    table: string,
    columns: readonly string[],
    entityMappers: IMapper<string, string> = new IdentityMapper(),
    propertyMappers: IMapper<string, string> = new IdentityMapper(),
  ) {
    super(table, columns);
    this.mappings = new StatementMappers(entityMappers, propertyMappers);
  }

  insert = (fields: Field[]): InsertStatement =>
    this.mappings.apply(new InsertStatement(this.table, fields), same) as InsertStatement;

  update = (
    fields: Field[],
    where?: IWhere,
    cell: (name: string, value: Cell) => Cell = same,
  ): UpdateStatement =>
    this.mappings.apply(new UpdateStatement(this.table, fields, where), cell) as UpdateStatement;

  delete = (where?: IWhere, cell: (name: string, value: Cell) => Cell = same): DeleteStatement =>
    this.mappings.apply(new DeleteStatement(this.table, where), cell) as DeleteStatement;

  selectInsertedId = (column: string): SelectInsertedIdStatement =>
    this.mappings.apply(new SelectInsertedIdStatement(this.table, column), same) as SelectInsertedIdStatement;
}
