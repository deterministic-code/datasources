import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
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
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export type Sourced<T> = { item: T; source?: Row };

export const tracked = Symbol("tracked");

type Tracked<T> = {
  [tracked](
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>>;
};

export async function* readTracked<T>(
  repo: IReadonlyRepository<T>,
  query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
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

const isRepository = <T>(
  value: Constructable<T> | IConnection | BuiltRepository<T> | IReadonlyRepository<T>,
): value is IReadonlyRepository<T> =>
  typeof value === "object" &&
  value !== null &&
  "read" in value &&
  typeof value.read === "function" &&
  !("createCommand" in value) &&
  !isBuilt(value);

export class ReadonlyRepository<T> implements IReadonlyRepository<T>, IEnriched<T> {
  readonly type: Constructable<T>;
  protected connection?: IConnection;
  readonly rowConverter?: TypeConverter<T, Cells>;
  readonly mapper?: IMapper<T, T>;
  private readonly columnMappings: IMapper<string, string> = new IdentityMapper();
  readonly statements?: RepositoryStatements;
  protected readonly middlewares: ICommandMiddleware[] = [];
  readonly tree: IRepositoryTree;
  protected innerRepo?: IReadonlyRepository<T>;
  private readonly strategy: IReadonlyEnrichmentStrategy<T>;

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: ReadonlyRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(inner: IReadonlyRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T> | IReadonlyRepository<T>,
    connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T> | IReadonlyEnrichmentStrategy<T>,
  ) {
    if (isRepository(typeOrConnectionOrBuilt)) {
      this.type = typeOrConnectionOrBuilt.type;
      this.innerRepo = typeOrConnectionOrBuilt;
      this.strategy =
        (connectionOrOptions as IReadonlyEnrichmentStrategy<T> | undefined) ??
        new DefaultReadonlyEnrichmentStrategy<T>();
      this.tree = new RepositoryTree(
        new RepositoryTreeNode(typeOrConnectionOrBuilt, "", undefined, undefined, undefined, this.strategy),
      );
      return;
    }
    const built = builtOf(typeOrConnectionOrBuilt, connectionOrOptions as IConnection | ReadonlyRepositoryOptions<T>);
    this.strategy = new DefaultReadonlyEnrichmentStrategy<T>();
    this.tree = new RepositoryTree(
      new RepositoryTreeNode(this, "", undefined, undefined, undefined, this.strategy),
    );
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
    nest?: (node: IRepositoryTreeNode) => void,
  ): this {
    const child = this.tree.parent.include(via, into, from, by, select, many);
    nest?.(child);
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
  ): this {
    this.tree.parent.includeLinked(via, into, from, by, link, parent, related);
    return this;
  }

  enrich = async (items: T[]): Promise<T[]> => {
    await this.strategy.enrich(this.tree, items);
    return items;
  };

  async *read(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): AsyncIterable<T> {
    for await (const sourced of this[tracked](query)) yield sourced.item;
  }

  async *[tracked](
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    const origin = this.innerRepo ? readTracked(this.innerRepo, query) : this.rows(query);
    if (this.innerRepo || this.enriched) {
      yield* this.sourced(origin);
      return;
    }
    yield* origin;
  }

  private async *sourced(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of pairs) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await this.strategy.enrich(this.tree, items, sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }

  private async *rows(
    query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement,
  ): AsyncIterable<Sourced<T>> {
    const statement =
      query instanceof ExecuteStoredProcedureStatement ? query : this.readQuery(query);
    const parameters =
      statement instanceof ExecuteStoredProcedureStatement
        ? statement.parameters
        : (statement.where?.parameters ?? statement.parameters);
    const reader = await executeReader(this.command(), statement, parameters);
    try {
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
    } finally {
      await reader.dispose();
    }
  }

  async readSingle(query?: ISelectStatement | IWhere | IExecuteStoredProcedureStatement): Promise<T | undefined> {
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

  protected readQuery = (query?: ISelectStatement | IWhere): ISelectStatement =>
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
