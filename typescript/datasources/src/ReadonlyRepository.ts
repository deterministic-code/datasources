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
import { createReadonlyRepositoryOptions } from "./createReadonlyRepositoryOptions.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { ReadonlyRepositoryBuilder } from "./ReadonlyRepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
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

const storedEnrichments = new WeakMap<object, StoredEnrichment[]>();

export const enrichmentsOf = (host: object): readonly StoredEnrichment[] | undefined =>
  storedEnrichments.get(host);

export const isBuilt = <T>(
  value: IConnection | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "objectConverter" in value && "propertyMappings" in value;

export const builtOf = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T>,
): BuiltRepository<T> | undefined =>
  typeof typeOrConnectionOrBuilt === "function"
    ? connectionOrOptions === undefined
      ? undefined
      : new ReadonlyRepositoryBuilder<T>().parts(
          connectionOrOptions as IConnection,
          createReadonlyRepositoryOptions(typeOrConnectionOrBuilt),
        )
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : new ReadonlyRepositoryBuilder<T>().parts(
          typeOrConnectionOrBuilt,
          connectionOrOptions as ReadonlyRepositoryOptions<T>,
        );

export class ReadonlyRepository<T> implements IReadonlyRepository<T>, IEnriched<T> {
  readonly type: Constructable<T>;
  private connection?: IConnection;
  protected rowConverter?: TypeConverter<Cells, Cells>;
  protected objectConverter?: TypeConverter<T, Cells>;
  private statementMappers = new StatementMappers(new IdentityMapper(), new IdentityMapper());
  private readonlyStatements?: ReadonlyRepositoryStatements;
  private readonly middlewares: ICommandMiddleware[] = [];

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: ReadonlyRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor();
  constructor(
    typeOrConnectionOrBuilt?: Constructable<T> | IConnection | BuiltRepository<T>,
    connectionOrOptions?: IConnection | ReadonlyRepositoryOptions<T>,
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
      new StoredEnrichment<T & object>(
        enrichment,
        (parents, sources) => chosen.apply(via, enrichment, parents, sources),
      ),
    );
    return this;
  }

  async *read(query?: ReadQuery, parameters?: Parameter[]): AsyncIterable<T> {
    for await (const sourced of this.readTracked(query, parameters)) yield sourced.item;
  }

  async readSingle(query?: ReadQuery, parameters?: Parameter[]): Promise<T | undefined> {
    for await (const item of this.read(query, parameters)) return item;
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

  protected column = (property: string): string => this.statementMappers.column(property);

  protected cell = (property: string, value: Cell, fieldsApplied: boolean): Cell => {
    const field =
      this.objectConverter instanceof MetaDataRowTypeConverter
        ? this.objectConverter.fields[property]
        : undefined;
    return field && !fieldsApplied ? field.reverseConvert(value) : value;
  };

  private entries = (): StoredEnrichment<T & object>[] =>
    storedEnrichments.get(this) as StoredEnrichment<T & object>[];

  private async *readTracked(query?: ReadQuery, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    const statement = this.statementMappers.apply(
      query instanceof ExecuteStoredProcedureStatement ? query : this.readQuery(query),
      (name, value) => this.cell(name, value, false),
    );
    const listed =
      parameters ??
      (statement instanceof SelectStatement ? statement.where?.parameters : undefined);
    yield* this.apply(this.readRows(statement, listed));
  }

  private async *apply(pairs: AsyncIterable<Sourced<T>>): AsyncIterable<Sourced<T>> {
    if (!this.enriched) {
      yield* pairs;
      return;
    }
    yield* this.sourced(pairs);
  }

  private async *readSources(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Cells> {
    const statement = query ?? this.readonlyStatements!.select();
    const reader = await executeReader(this.command(), statement, parameters);
    try {
      while (await reader.next()) {
        yield this.columnToPropertyName(
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

  private convertObject = (row: Cells): T =>
    this.objectConverter instanceof MetaDataRowTypeConverter
      ? this.objectConverter.convert(row, this.type)
      : this.objectConverter!.convert(row);

  private async *readRows(query?: IStatement, parameters?: Parameter[]): AsyncIterable<Sourced<T>> {
    for await (const renamed of this.readSources(query, parameters))
      yield {
        item: this.convertObject(this.rowConverter ? this.rowConverter.convert(renamed) : renamed),
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
    await this.fill(items as (T & object)[], sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }

  private fill = async (parents: (T & object)[], sources?: (Row | undefined)[]): Promise<void> => {
    for (const entry of this.entries()) await entry.apply(parents, sources);
  };

  private columnToPropertyName = (row: Cells): Cells =>
    Object.fromEntries(Object.entries(row).map(([key, value]) => [this.statementMappers.property(key), value]));

  private readQuery = (query?: ISelectStatement | IWhere): ISelectStatement | undefined =>
    !query
      ? this.readonlyStatements?.select()
      : "where" in query
        ? query
        : this.readonlyStatements
          ? this.readonlyStatements.select(
              this.statementMappers.applyWhere(query, (name, cell) => this.cell(name, cell, false))!,
            )
          : new SelectStatement(query.getText(), query);
}
