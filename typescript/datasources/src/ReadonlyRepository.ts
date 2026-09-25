import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { Where } from "./statements/Where.ts";
import { createRepositoryOptions } from "./createRepositoryOptions.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export type Sourced<T> = { item: T; source?: Row };

export const tracked = Symbol("tracked");

type Tracked<T> = {
  [tracked](
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>>;
};

export async function* readTracked<T>(
  repo: IReadonlyRepository<T>,
  query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
): AsyncIterable<Sourced<T>> {
  const read = (repo as Partial<Tracked<T>>)[tracked];
  if (read) {
    yield* read.call(repo, query);
    return;
  }
  for await (const item of repo.read(query)) yield { item };
}

export async function* withoutSource<T>(items: AsyncIterable<T>): AsyncIterable<Sourced<T>> {
  for await (const item of items) yield { item };
}

export const isBuilt = <T>(
  value: IConnection | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "rowConverter" in value && "rowsMapper" in value;

export const builtOf = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T>,
): BuiltRepository<T> | undefined =>
  typeof typeOrConnectionOrBuilt === "function"
    ? connectionOrOptions === undefined
      ? undefined
      : parts(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnectionOrBuilt))
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : parts(typeOrConnectionOrBuilt, connectionOrOptions as ReadonlyRepositoryOptions<T>);

export class ReadonlyRepository<T> implements IReadonlyRepository<T> {
  readonly type: Constructable<T>;
  protected connection?: IConnection;
  readonly rowConverter?: TypeConverter<T, Cells>;
  readonly mapper?: IMapper<T, T>;
  private readonly columnMappings: IMapper<string, string> = new IdentityMapper();
  readonly statements?: RepositoryStatements;
  protected readonly middlewares: ICommandMiddleware[] = [];

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: ReadonlyRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
    connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T>,
  ) {
    const built = builtOf(typeOrConnectionOrBuilt, connectionOrOptions);
    if (!built) {
      this.type = typeOrConnectionOrBuilt as Constructable<T>;
      return;
    }
    this.type = built.type;
    this.connection = built.connection;
    this.rowConverter = built.rowConverter;
    this.mapper = built.rowsMapper;
    this.columnMappings = built.columnMappings;
    this.statements = built.statements;
  }

  addMiddleware = (middleware: ICommandMiddleware): this => {
    this.middlewares.push(middleware);
    return this;
  };

  async *read(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T> {
    for await (const sourced of this[tracked](query)) yield sourced.item;
  }

  async *[tracked](
    query?: IQuery | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    const statement =
      query instanceof ExecuteStoredProcedureStatement ? query : this.readQuery(query);
    const parameters =
      statement instanceof ExecuteStoredProcedureStatement
        ? statement.parameters
        : (statement.where?.parameters ?? statement.parameters);
    const reader = await executeReader(this.command(), statement, parameters);
    while (await reader.next()) {
      const renamed = this.renamed(
        reader.current ??
          (() => {
            throw new DatasourceError("Reader current must be a row");
          })(),
      );
      yield {
        item: this.mapper!.map(this.rowConverter!.convert(renamed)),
        source: renamed,
      };
    }
  }

  async readSingle(query?: IQuery | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined> {
    for await (const item of this.read(query)) return item;
    return undefined;
  }

  protected command = (): ICommand =>
    this.middlewares.reduce(
      (command, middleware) => command.addMiddleware(middleware),
      this.connection!.createCommand(),
    );

  private renamed = (row: Cells): Cells =>
    Object.fromEntries(
      Object.entries(row).map(([key, value]) => [this.columnMappings.reverseMap(key), value]),
    );

  protected readQuery = (query?: IQuery | IWhere): IQuery =>
    !query
      ? this.statements!.query()
      : "where" in query
        ? query
        : this.statements!.query(this.stored(query));

  protected stored = (where: IWhere, fieldsApplied = false): IWhere =>
    where instanceof Where
      ? where.map(this.column, (name, cell) => this.cell(name, cell, fieldsApplied))
      : where;

  protected column = (property: string): string => this.columnMappings.map(property);

  protected cell = (property: string, value: Cell, fieldsApplied: boolean): Cell => {
    const field =
      this.rowConverter instanceof RowTypeConverter ? this.rowConverter.fields[property] : undefined;
    return field && !fieldsApplied ? field.reverseConvert(value) : value;
  };
}
