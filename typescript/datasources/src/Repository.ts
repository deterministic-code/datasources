import { DatasourceError } from "./DatasourceError.ts";
import type { ICommand } from "./ICommand.ts";
import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./IQuery.ts";
import type { IRepository } from "./IRepository.ts";
import type { IResponse } from "./IResponse.ts";
import type { IWhere } from "./IWhere.ts";
import { StatementBuilder, WhereBuilder } from "./builders/index.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { FieldConverter } from "./converters/FieldConverter.ts";
import { TypeConverter } from "./converters/TypeConverter.ts";
import type { TypeMapping } from "./converters/TypeMapping.ts";

export type RepositoryOptions<T extends Cells> = {
  datasourceName: string;
  connection: IConnection;
  ids?: string[];
  assignIdentityOnAdd?: boolean;
  schema?: Record<string, string>;
  converters?: Record<string, FieldConverter>;
  names?: Record<string, string>;
  mapping?: TypeMapping<T>;
  type?: Constructable<T>;
};

const fieldNamesOf = <T>(type: Constructable<T>): string[] =>
  Object.keys(new type() as object);

const defaultId = (fields: string[]): string[] =>
  fields.includes("id") ? ["id"] : fields.includes("Id") ? ["Id"] : [];

export const createRepositoryOptions = <T extends Cells>(
  type: Constructable<T>,
  connection: IConnection,
): RepositoryOptions<T> => ({
  type,
  datasourceName: type.name,
  ids: defaultId(fieldNamesOf(type)),
  connection,
});

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

export class Repository<T extends Cells> implements IRepository<T> {
  readonly connection: IConnection;
  private readonly options: RepositoryOptions<T>;
  private readonly mapping: TypeMapping<T>;
  private readonly ids: Set<string>;

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(options: RepositoryOptions<T>);
  constructor(
    typeOrOptions: Constructable<T> | RepositoryOptions<T>,
    connection?: IConnection,
  ) {
    this.options =
      connection === undefined
        ? (typeOrOptions as RepositoryOptions<T>)
        : createRepositoryOptions(typeOrOptions as Constructable<T>, connection);
    this.ids = new Set(this.options.ids ?? ["id"]);
    this.connection = this.options.connection;
    this.mapping =
      this.options.mapping ??
      TypeConverter.mapping<T>({
        source: this.options.connection.provider,
        destination: "typescript",
        type: this.options.type,
        schema: this.options.schema,
        converters: this.options.converters,
        names: this.options.names,
      });
  }

  getSingle = async (query?: IQuery): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery): AsyncIterable<T> {
    const statement =
      query ?? StatementBuilder.createQuery(this.options.datasourceName, this.fields());
    const reader = await this.command().executeReader(
      statement.getText(),
      statement.where?.parameters ?? statement.parameters,
    );
    while (await reader.next()) yield this.mapping.to(asRow(reader.current as object));
  }

  add = async (obj: T): Promise<IResponse> => {
    const fields = fieldsOf(
      this.mapping.from(obj),
      (col) => this.options.assignIdentityOnAdd === false || !this.ids.has(col),
    );
    return this.command().execute(
      StatementBuilder.createInsert(this.options.datasourceName, fields).getText(),
      fields,
    );
  };

  update = async (obj: T, byWhere?: IWhere): Promise<IResponse> => {
    const record = this.mapping.from(obj);
    const where = byWhere ?? this.idWhere(record);
    const fields = fieldsOf(record, (col) => !this.ids.has(col));
    if (fields.length === 0)
      throw new DatasourceError("Update requires at least one non-Id field");
    return this.command().execute(
      StatementBuilder.createUpdate(this.options.datasourceName, fields, where).getText(),
      [...fields, ...(where.parameters ?? [])],
    );
  };

  delete = async (obj: T, byWhere?: IWhere): Promise<IResponse> => {
    const where = byWhere ?? this.idWhere(this.mapping.from(obj));
    return this.command().execute(
      StatementBuilder.createDelete(this.options.datasourceName, where).getText(),
      where.parameters,
    );
  };

  private command = (): ICommand => this.options.connection.createCommand();

  private fields = (): string[] => Object.keys(this.mapping.schema).sort();

  private idWhere = (record: Cells): IWhere =>
    WhereBuilder.and(
      ...this.idColumns().map((col) => {
        if (!Object.prototype.hasOwnProperty.call(record, col))
          throw new DatasourceError(`Missing Id value "${col}"`);
        return WhereBuilder.equals(col, record[col] as Scalar);
      }),
    );

  private idColumns = (): string[] => this.options.ids ?? ["id"];
}
