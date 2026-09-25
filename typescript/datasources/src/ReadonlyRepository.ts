import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { Parameter } from "./Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IStatement } from "./statements/IStatement.ts";
import type { IReadonlyRepository, ReadQuery } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";
import { Where } from "./statements/Where.ts";
import { createRepositoryOptions } from "./createRepositoryOptions.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export type Sourced<T> = { item: T; source?: Row };

export const tracked = Symbol("tracked");

type Tracked<T> = {
  [tracked](query?: ReadQuery, parameters?: Parameter[]): AsyncIterable<Sourced<T>>;
};

export async function* readTracked<T>(
  repo: IReadonlyRepository<T>,
  query?: ReadQuery,
  parameters?: Parameter[],
): AsyncIterable<Sourced<T>> {
  const read = (repo as Partial<Tracked<T>>)[tracked];
  if (read) {
    yield* read.call(repo, query, parameters);
    return;
  }
  for await (const item of repo.read(query, parameters)) yield { item };
}

export async function* withoutSource<T>(items: AsyncIterable<T>): AsyncIterable<Sourced<T>> {
  for await (const item of items) yield { item };
}

export const isBuilt = <T>(
  value: IConnection | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "rowConverter" in value && "rowsMapper" in value;

const isResultMapper = <T>(
  value: Constructable<T> | IConnection | BuiltRepository<T> | IMapper<Iterable<Cells>, T[]>,
): value is IMapper<Iterable<Cells>, T[]> =>
  typeof value === "object" &&
  value !== null &&
  "map" in value &&
  "reverseMap" in value &&
  !("createCommand" in value) &&
  !isBuilt(value as BuiltRepository<T>);

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

export class ReadonlyRepository<T> implements IReadonlyRepository<T>, IEnriched<T> {
  readonly type: Constructable<T>;
  protected connection?: IConnection;
  readonly rowConverter?: TypeConverter<T, Cells>;
  readonly mapper?: IMapper<T, T>;
  private readonly result?: IMapper<Iterable<Cells>, T[]>;
  private readonly columnMappings: IMapper<string, string> = new IdentityMapper();
  readonly statements?: RepositoryStatements;
  protected readonly middlewares: ICommandMiddleware[] = [];
  readonly tree: IRepositoryTree;

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: ReadonlyRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(result?: IMapper<Iterable<Cells>, T[]>);
  constructor(
    typeOrConnectionOrBuiltOrResult?:
      | Constructable<T>
      | IConnection
      | BuiltRepository<T>
      | IMapper<Iterable<Cells>, T[]>,
    connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T>,
  ) {
    this.tree = new RepositoryTree(
      new RepositoryTreeNode(this, "", undefined, undefined, undefined, new DefaultReadonlyEnrichmentStrategy<T & object>()),
    );
    if (
      typeOrConnectionOrBuiltOrResult === undefined ||
      isResultMapper(typeOrConnectionOrBuiltOrResult)
    ) {
      this.type = untyped();
      this.result = typeOrConnectionOrBuiltOrResult;
      return;
    }
    const built = builtOf(typeOrConnectionOrBuiltOrResult, connectionOrOptions);
    if (!built) {
      this.type = typeOrConnectionOrBuiltOrResult as Constructable<T>;
      return;
    }
    this.type = built.type;
    this.connection = built.connection;
    this.rowConverter = built.rowConverter;
    this.mapper = built.rowsMapper;
    this.result = built.result;
    this.columnMappings = built.columnMappings;
    this.statements = built.statements;
  }

  protected get enriched(): boolean {
    return this.tree.parent.items.length > 0;
  }

  addMiddleware = (middleware: ICommandMiddleware): this => {
    this.middlewares.push(middleware);
    return this;
  };

  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this {
    (this.tree.parent as RepositoryTreeNode).includeWith(
      via,
      into,
      from,
      by,
      select,
      many,
      strategy ?? new DefaultReadonlyEnrichmentStrategy<T & object>(),
    );
    return this;
  }

  addLinked<TRelated extends object, TLink extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    link: ICRUDRepository<TLink>,
    parent: keyof TLink & string,
    related: keyof TLink & string,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this {
    (this.tree.parent as RepositoryTreeNode).includeLinkedWith(
      via,
      into,
      from,
      by,
      link,
      parent,
      related,
      strategy ?? new DefaultReadonlyEnrichmentStrategy<T & object>(),
    );
    return this;
  }

  enrich = async (items: T[]): Promise<void> => {
    await (this.tree.parent as RepositoryTreeNode).enrichFrom(items as object[]);
  };

  async *read(query?: ReadQuery, parameters?: Parameter[]): AsyncIterable<T> {
    for await (const sourced of this[tracked](query, parameters)) yield sourced.item;
  }

  async *[tracked](query?: ReadQuery, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    const statement = query instanceof ExecuteStoredProcedureStatement ? query : this.readQuery(query);
    const listed =
      parameters ??
      (statement instanceof ExecuteStoredProcedureStatement ? undefined : statement?.where?.parameters);
    yield* this.apply(this.readItems(statement, listed));
  }

  protected async *readItems(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    yield* this.result ? this.collapse(query, parameters) : this.readRows(query, parameters);
  }

  private async *apply(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    if (!this.enriched) {
      yield* pairs;
      return;
    }
    yield* this.sourced(pairs);
  }

  private async *collapse(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    const rows: Cells[] = [];
    for await (const row of this.readSources(query, parameters)) rows.push(row);
    for (const item of this.result!.map(rows)) yield { item };
  }

  protected async *readSources(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Cells> {
    const reader = await executeReader(this.command(), query ?? this.statements!.query(), parameters);
    try {
      while (await reader.next()) {
        yield this.renamed(
          reader.current ??
            (() => {
              throw new DatasourceError("Reader current must be a row");
            })(),
        );
      }
    } finally {
      await reader.dispose();
    }
  }

  private async *readRows(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    for await (const renamed of this.readSources(query, parameters))
      yield {
        item: this.mapper!.map(this.rowConverter!.convert(renamed)),
        source: renamed,
      };
  }

  private async *sourced(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of pairs) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await (this.tree.parent as RepositoryTreeNode).enrichFrom(items as object[], sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }

  async readSingle(query?: ReadQuery, parameters?: Parameter[]): Promise<T | undefined> {
    for await (const item of this.read(query, parameters)) return item;
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

  protected readQuery = (query?: ISelectStatement | IWhere): ISelectStatement | undefined =>
    !query
      ? this.statements?.query()
      : "where" in query
        ? query
        : this.statements
          ? this.statements.query(this.stored(query))
          : new SelectStatement(query.getText(), query);

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
