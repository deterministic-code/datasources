import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Operator } from "./statements/Operator.ts";
import { WhereClause } from "./statements/WhereClause.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { builtOf, ReadonlyRepository } from "./ReadonlyRepository.ts";
import { DatasourceError } from "./DatasourceError.ts";

const fieldsOf = (record: Cells, include: (col: string) => boolean): Field[] =>
  Object.entries(record)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: value as Scalar }));

export class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T> {
  private readonly ids: Set<string>;
  private readonly assignIdentityOnCreate: boolean;

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(options: RepositoryBuilderOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
    connection?: IConnection,
  ) {
    const built = builtOf(typeOrOptions, connection)!;
    super(built);
    this.ids = new Set(built.ids);
    this.assignIdentityOnCreate = built.assignIdentityOnCreate;
  }

  create = async (item: T): Promise<Response> => {
    const fields = fieldsOf(
      this.reverseConvert(item),
      (col) => !this.assignIdentityOnCreate || !this.ids.has(col),
    );
    return this.command().execute(this.statements!.insert(fields).getText(), fields);
  };

  update = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ?? this.idWhere(record);
    const fields = fieldsOf(record, (col) => !this.ids.has(col));
    if (fields.length === 0)
      throw new DatasourceError("Update requires at least one non-Id field");
    return this.command().execute(this.statements!.update(fields, where).getText(), [
      ...fields,
      ...(where.parameters ?? []),
    ]);
  };

  delete = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const where = byWhere ?? this.idWhere(this.reverseConvert(item));
    return this.command().execute(
      this.statements!.delete(where).getText(),
      where.parameters,
    );
  };

  private reverseConvert = (item: T): Cells =>
    this.rowConverter!.reverseConvert(this.mapper!.reverseMap(item));

  private idWhere = (record: Cells): IWhere => {
    const parts: IExpression[] = [...this.ids].map((col) => {
      if (!Object.prototype.hasOwnProperty.call(record, col))
        throw new DatasourceError(`Missing Id value "${col}"`);
      return new Operator.Equals(col, record[col] as Scalar);
    });
    return new WhereClause(parts.reduce((left, right) => new Operator.And(left, right)));
  };
}
