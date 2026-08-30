import { DatasourceError } from "./DatasourceError.ts";
import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./IQuery.ts";
import type { IRepository } from "./IRepository.ts";
import type { IResponse } from "./IResponse.ts";
import type { IWhere } from "./IWhere.ts";
import { WhereBuilder } from "./builders/index.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { TypeMapping } from "./converters/TypeMapping.ts";
import type { StringSourceDestinationMapper } from "./mappers/StringSourceDestinationMapper.ts";
import { createRepositoryBuilderOptions } from "./createRepositoryBuilderOptions.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import {
  parts,
  type BuiltRepository,
  type RepositoryCommands,
  type RepositoryStatements,
} from "./RepositoryBuilder.ts";

const mapKeys = (row: Cells, name: (key: string) => string): Cells =>
  Object.fromEntries(Object.entries(row).map(([key, value]) => [name(key), value]));

const asRow = (current: object | null | undefined): Cells =>
  current && typeof current === "object" && !Array.isArray(current)
    ? (current as Cells)
    : (() => {
        throw new DatasourceError("Reader current must be a row dictionary");
      })();

const fieldsOf = (record: Cells, include: (col: string) => boolean): Field[] =>
  Object.entries(record)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: value as Scalar }));

const isBuilt = <T extends Cells>(
  value: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "commands" in value && "converters" in value && "statements" in value;

export class Repository<T extends Cells> implements IRepository<T> {
  readonly commands: RepositoryCommands;
  readonly converters: TypeMapping<T>;
  readonly mapping: StringSourceDestinationMapper;
  readonly statements: RepositoryStatements;
  private readonly ids: Set<string>;
  private readonly assignIdentityOnAdd: boolean;

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(options: RepositoryBuilderOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
    connection?: IConnection,
  ) {
    const built = isBuilt(typeOrOptions)
      ? typeOrOptions
      : parts(
          connection === undefined
            ? (typeOrOptions as RepositoryBuilderOptions<T>)
            : createRepositoryBuilderOptions(typeOrOptions as Constructable<T>, connection),
        );
    this.commands = built.commands;
    this.converters = built.converters;
    this.mapping = built.mapping;
    this.statements = built.statements;
    this.ids = new Set(built.ids);
    this.assignIdentityOnAdd = built.assignIdentityOnAdd;
  }

  get connection(): IConnection {
    return this.commands.get.connection;
  }

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    const statement = this.readQuery(query);
    const reader = await this.commands.get.executeReader(
      statement.getText(),
      statement.where?.parameters ?? statement.parameters,
    );
    while (await reader.next())
      yield this.converters.to(mapKeys(asRow(reader.current as object), this.mapping.to));
  }

  add = async (obj: T): Promise<IResponse> => {
    const fields = fieldsOf(
      this.sql(obj),
      (col) => !this.assignIdentityOnAdd || !this.ids.has(col),
    );
    return this.commands.add.execute(this.statements.insert(fields).getText(), fields);
  };

  update = async (obj: T, byWhere?: IWhere): Promise<IResponse> => {
    const record = this.sql(obj);
    const where = byWhere ?? this.idWhere(record);
    const fields = fieldsOf(record, (col) => !this.ids.has(col));
    if (fields.length === 0)
      throw new DatasourceError("Update requires at least one non-Id field");
    return this.commands.update.execute(this.statements.update(fields, where).getText(), [
      ...fields,
      ...(where.parameters ?? []),
    ]);
  };

  delete = async (obj: T, byWhere?: IWhere): Promise<IResponse> => {
    const where = byWhere ?? this.idWhere(this.sql(obj));
    return this.commands.delete.execute(
      this.statements.delete(where).getText(),
      where.parameters,
    );
  };

  private sql = (obj: T): Cells => mapKeys(this.converters.from(obj), this.mapping.from);

  private readQuery = (query?: IQuery | IWhere): IQuery =>
    !query ? this.statements.query() : "where" in query ? query : this.statements.query(query);

  private idWhere = (record: Cells): IWhere =>
    WhereBuilder.and(
      ...[...this.ids].map((col) => {
        if (!Object.prototype.hasOwnProperty.call(record, col))
          throw new DatasourceError(`Missing Id value "${col}"`);
        return WhereBuilder.equals(col, record[col] as Scalar);
      }),
    );
}
