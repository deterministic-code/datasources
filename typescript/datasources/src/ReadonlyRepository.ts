import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { Parameter } from "./Parameter.ts";
import type { ISelectStatement } from "./statements/ISelectStatement.ts";
import type { IStatement } from "./statements/IStatement.ts";
import type { IReadonlyRepository, ReadQuery } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { SelectStatement } from "./statements/SelectStatement.ts";
import { createReadonlyRepositoryMappingOptions } from "./createReadonlyRepositoryMappingOptions.ts";
import type { ReadonlyRepositoryMappingOptions } from "./ReadonlyRepositoryMappingOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { ReadonlyRepositoryBuilder } from "./ReadonlyRepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import type { ReadonlyRepositoryStatements } from "./ReadonlyRepositoryStatements.ts";
import { StatementMappers } from "./StatementMappers.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import { StoredEnrichment } from "./StoredEnrichment.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export type Sourced<T> = { item: T; source?: Row };

const storedEnrichments = new WeakMap<object, StoredEnrichment<never>[]>();

export const enrichmentsOf = <T>(host: object): readonly StoredEnrichment<T>[] | undefined =>
  storedEnrichments.get(host) as readonly StoredEnrichment<T>[] | undefined;

export const isBuilt = <T>(
  value: IConnection | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "objectConverter" in value && "propertyMappings" in value;

export const builtOf = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | ReadonlyRepositoryMappingOptions<T>,
): BuiltRepository<T> | undefined =>
  typeof typeOrConnectionOrBuilt === "function"
    ? connectionOrOptions === undefined
      ? undefined
      : new ReadonlyRepositoryBuilder<T>().parts(
          connectionOrOptions as IConnection,
          createReadonlyRepositoryMappingOptions(typeOrConnectionOrBuilt),
        )
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : new ReadonlyRepositoryBuilder<T>().parts(
          typeOrConnectionOrBuilt,
          connectionOrOptions as ReadonlyRepositoryMappingOptions<T>,
        );

export class ReadonlyRepository<T> implements IReadonlyRepository<T>, IEnriched<T> {
  readonly type: Constructable<T>;
  private connection?: IConnection;
  protected rowConverter?: TypeConverter<Cells, Cells>;
  protected objectConverter?: TypeConverter<T, Cells>;
  private entityMappers: IMapper<string, string> = new IdentityMapper();
  private propertyMappers: IMapper<string, string> = new IdentityMapper();
  private statementMappers = new StatementMappers(new IdentityMapper(), new IdentityMapper());
  private readonlyStatements?: ReadonlyRepositoryStatements;
  private readonly middlewares: ICommandMiddleware[] = [];

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: ReadonlyRepositoryMappingOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor();
  constructor(
    typeOrConnectionOrBuilt?: Constructable<T> | IConnection | BuiltRepository<T>,
    connectionOrOptions?: IConnection | ReadonlyRepositoryMappingOptions<T>,
  ) {
    storedEnrichments.set(this, []);
    if (typeOrConnectionOrBuilt === undefined) {
      this.type = untyped();
      return;
    }
    const built = builtOf(typeOrConnectionOrBuilt, connectionOrOptions);
    if (!built) {
      this.type = typeOrConnectionOrBuilt as Constructable<T>;
      return;
    }
    this.type = built.type;
    this.connection = built.connection;
    this.rowConverter = built.rowConverter;
    this.objectConverter = built.objectConverter;
    this.entityMappers = built.entityMapper;
    this.propertyMappers = built.propertyMappings;
    this.statementMappers = new StatementMappers(built.entityMapper, built.propertyMappings);
    this.readonlyStatements = built.statements;
  }

  addMiddleware = (middleware: ICommandMiddleware): this => {
    this.middlewares.push(middleware);
    return this;
  };

  addEnriched<TRelated extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this {
    const chosen = strategy ?? new DefaultReadonlyEnrichmentStrategy<T & object>();
    this.entries().push(
      new StoredEnrichment<T, TRelated>(
        enrichment,
        (parents, sources) => chosen.apply(via, enrichment, parents as (T & object)[], sources),
        undefined,
        undefined,
        via,
      ) as unknown as StoredEnrichment<T>,
    );
    return this;
  }

  async *read(
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): AsyncIterable<T> {
    for await (const sourced of this.readTracked(query, parameters, options)) yield sourced.item;
  }

  async readSingle(
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): Promise<T | undefined> {
    for await (const item of this.read(query, parameters, options)) return item;
    return undefined;
  }

  protected get enriched(): boolean {
    return this.entries().length > 0;
  }

  protected command = (): ICommand =>
    this.middlewares.reduce(
      (command, middleware) => command.addMiddleware(middleware),
      this.connection!.createCommand(),
    );

  private ensureOpen = async (): Promise<void> => {
    const connection = this.connection;
    if (!connection) throw new DatasourceError("Connection required");
    if (!connection.isOpen) await connection.open();
  };

  protected column = (property: string): string => this.statementMappers.column(property);

  protected cell = (
    property: string,
    value: Cell,
    fieldsApplied: boolean,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): Cell => {
    const objects = options?.objectConverter ?? this.objectConverter;
    const field =
      objects instanceof MetaDataRowTypeConverter ? objects.fields[property] : undefined;
    return field && !fieldsApplied ? field.reverseConvert(value) : value;
  };

  private entries = (): StoredEnrichment<T>[] =>
    storedEnrichments.get(this) as unknown as StoredEnrichment<T>[];

  private mappers = (options?: ReadonlyRepositoryMappingOptions<T>): StatementMappers =>
    options?.entityMapper === undefined && options?.propertyMappings === undefined
      ? this.statementMappers
      : new StatementMappers(
          options?.entityMapper ?? this.entityMappers,
          options?.propertyMappings ?? this.propertyMappers,
        );

  private rowConverterOf = (options?: ReadonlyRepositoryMappingOptions<T>): TypeConverter<Cells, Cells> | undefined =>
    options?.rowConverter ?? this.rowConverter;

  private objects = (options?: ReadonlyRepositoryMappingOptions<T>): TypeConverter<T, Cells> =>
    options?.objectConverter ??
    this.objectConverter ??
    (() => {
      throw new DatasourceError("Object converter required");
    })();

  private async *readTracked(
    query?: ReadQuery,
    parameters?: Parameter[],
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): AsyncIterable<Sourced<T>> {
    const mappers = this.mappers(options);
    const statement = mappers.apply(
      query instanceof ExecuteStoredProcedureStatement ? query : this.readQuery(query, mappers, options),
      (name, value) => this.cell(name, value, false, options),
    );
    const listed =
      parameters ??
      (statement instanceof SelectStatement ? statement.where?.parameters : undefined);
    yield* this.apply(this.readItems(statement, listed, mappers, this.rowConverterOf(options), this.objects(options)));
  }

  private async *apply(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    if (!this.enriched) {
      yield* pairs;
      return;
    }
    yield* this.sourced(pairs);
  }

  private convertObject = (row: Cells, objects: TypeConverter<T, Cells>): T =>
    objects instanceof MetaDataRowTypeConverter
      ? objects.convert(row, this.type)
      : objects.convert(row);

  private async *readItems(
    query: IStatement | undefined,
    parameters: Parameter[] | undefined,
    mappers: StatementMappers,
    rows: TypeConverter<Cells, Cells> | undefined,
    objects: TypeConverter<T, Cells>,
  ): AsyncIterable<Sourced<T>> {
    const statement = query ?? this.readonlyStatements!.select();
    await this.ensureOpen();
    const reader = await executeReader(this.command(), statement, parameters);
    try {
      while (await reader.next()) {
        const renamed = this.columnToPropertyName(
          reader.current ??
            (() => {
              throw new DatasourceError("Reader current must be a row");
            })(),
          mappers,
        );
        yield {
          item: this.convertObject(rows ? rows.convert(renamed) : renamed, objects),
          source: renamed,
        };
      }
    } finally {
      await reader.dispose();
    }
  }

  private async *sourced(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of pairs) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await this.fill(items as (T & object)[], sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }

  private fill = async (parents: (T & object)[], sources?: (Row | undefined)[]): Promise<void> => {
    for (const entry of this.entries()) await entry.apply(parents, sources);
  };

  private columnToPropertyName = (row: Cells, mappers: StatementMappers): Cells =>
    Object.fromEntries(Object.entries(row).map(([key, value]) => [mappers.property(key), value]));

  private readQuery = (
    query: ISelectStatement | IWhere | undefined,
    mappers: StatementMappers,
    options?: ReadonlyRepositoryMappingOptions<T>,
  ): ISelectStatement | undefined =>
    !query
      ? this.readonlyStatements?.select()
      : "where" in query
        ? query
        : this.readonlyStatements
          ? this.readonlyStatements.select(query)
          : new SelectStatement({ where: query });
}
