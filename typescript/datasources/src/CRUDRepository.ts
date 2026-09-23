import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal } from "./statements/Literal.ts";
import { Operator } from "./statements/Operator.ts";
import { WhereClause } from "./statements/WhereClause.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { createRepositoryOptions, defaultId, fieldNamesOf } from "./createRepositoryOptions.ts";
import { parts } from "./RepositoryBuilder.ts";
import { isBuilt, ReadonlyRepository } from "./ReadonlyRepository.ts";
import { execute } from "./ICommand.ts";
import { DatasourceError } from "./DatasourceError.ts";

const fieldsOf = (record: Cells, include: (col: string) => boolean): Field[] =>
  Object.entries(record)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: value as Scalar }));

const apply = <T>(item: T, identity: T): void => {
  if (!item || !identity || typeof item !== "object" || typeof identity !== "object") return;
  for (const [key, value] of Object.entries(identity))
    if (value != null)
      (item as Record<string, Scalar>)[key === "id" && !("id" in item) && "Id" in item ? "Id" : key] =
        value as Scalar;
};

const asScalar = (value: Scalar | undefined): Scalar | undefined =>
  value == null ? undefined : value;

const crudBuilt = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
): BuiltRepository<T> =>
  typeof typeOrConnectionOrBuilt === "function"
    ? parts(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnectionOrBuilt))
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : parts(typeOrConnectionOrBuilt, withIds(connectionOrOptions as CRUDRepositoryOptions<T>));

const withIds = <T>(options: CRUDRepositoryOptions<T>): CRUDRepositoryOptions<T> =>
  "ids" in options
    ? options
    : {
        ...options,
        ids: options.type ? defaultId(fieldNamesOf(options.type)) : ["id"],
      };

export class CRUDRepository<T = Cells> extends ReadonlyRepository<T> implements ICRUDRepository<T> {
  private readonly createMapper?: IMapper<Scalar, T>;
  private readonly ids: Set<string>;

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
  ) {
    const built = crudBuilt(typeOrConnectionOrBuilt, connectionOrOptions);
    super(built);
    this.createMapper = built.create;
    this.ids = new Set(built.ids);
  }

  create = async (item: T): Promise<Response> => {
    const fields = fieldsOf(this.reverseConvert(item), (col) => !this.ids.has(col));
    const response = await execute(this.command(), this.statements!.insert(fields), fields);
    const scalar = asScalar(
      response.result as string | number | boolean | bigint | null | undefined,
    );
    if (scalar !== undefined && this.createMapper)
      apply(item, this.createMapper.map(scalar));
    return response;
  };

  update = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ?? this.idWhere(record);
    const fields = fieldsOf(record, (col) => !this.ids.has(col));
    if (fields.length === 0)
      throw new DatasourceError("Update requires at least one non-Id field");
    return execute(this.command(), this.statements!.update(fields, where), [
      ...fields,
      ...(where.parameters ?? []),
    ]);
  };

  delete = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ?? this.idWhere(record);
    return execute(
      this.command(),
      this.statements!.delete(where),
      where.parameters?.length ? where.parameters : fieldsOf(record, () => true),
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
    return new WhereClause(
      parts.length === 0
        ? new Operator.Equals(new Literal(1), new Literal(1))
        : parts.reduce((left, right) => new Operator.And(left, right)),
    );
  };
}
