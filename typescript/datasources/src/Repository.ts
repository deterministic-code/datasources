import { DatasourceError } from "./DatasourceError.ts";
import type { ICommand } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
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
import { createRepositoryBuilderOptions } from "./createRepositoryBuilderOptions.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { StatementMapper } from "./mappers/StatementMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

const fieldsOf = (record: Cells, include: (col: string) => boolean): Field[] =>
  Object.entries(record)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: value as Scalar }));

const isBuilt = <T>(
  value: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "rowConverter" in value && "statementMapper" in value;

export class Repository<T = Cells> implements IRepository<T> {
  private readonly connection: IConnection;
  readonly rowConverter: TypeConverter<T, Cells>;
  readonly statementsMapper: StatementMapper<T>;
  readonly mapper: IMapper<T, T>;
  readonly statements: RepositoryStatements;
  private readonly writeColumns: string[];
  private readonly middlewares: ICommandMiddleware[] = [];
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
    this.connection = built.connection;
    this.rowConverter = built.rowConverter;
    this.statementsMapper = built.statementMapper;
    this.mapper = built.statementMapper.rows;
    this.statements = built.statements;
    this.writeColumns = built.writeColumns;
    this.ids = new Set(built.ids);
    this.assignIdentityOnAdd = built.assignIdentityOnAdd;
  }

  use = (middleware: ICommandMiddleware): this => {
    this.middlewares.push(middleware);
    return this;
  };

  getSingle = async (query?: IQuery | IWhere): Promise<T | undefined> => {
    for await (const item of this.get(query)) return item;
    return undefined;
  };

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    const statement = this.readQuery(query);
    const reader = await this.command().executeReader(
      statement.getText(),
      statement.where?.parameters ?? statement.parameters,
    );
    while (await reader.next())
      yield this.mapper.map(
        this.rowConverter.convert(
          reader.current ??
            (() => {
              throw new DatasourceError("Reader current must be a row");
            })(),
        ),
      );
  }

  add = async (item: T): Promise<IResponse> => {
    const fields = fieldsOf(
      this.reverseConvert(item),
      (col) => this.inTable(col) && (!this.assignIdentityOnAdd || !this.ids.has(col)),
    );
    return this.command().execute(this.statements.insert(fields).getText(), fields);
  };

  update = async (item: T, byWhere?: IWhere): Promise<IResponse> => {
    const record = this.reverseConvert(item);
    const where = byWhere ?? this.idWhere(record);
    const fields = fieldsOf(record, (col) => this.inTable(col) && !this.ids.has(col));
    if (fields.length === 0)
      throw new DatasourceError("Update requires at least one non-Id field");
    return this.command().execute(this.statements.update(fields, where).getText(), [
      ...fields,
      ...(where.parameters ?? []),
    ]);
  };

  delete = async (item: T, byWhere?: IWhere): Promise<IResponse> => {
    const where = byWhere ?? this.idWhere(this.reverseConvert(item));
    return this.command().execute(
      this.statements.delete(where).getText(),
      where.parameters,
    );
  };

  private command = (): ICommand =>
    this.middlewares.reduce(
      (command, middleware) => command.use(middleware),
      this.connection.createCommand(),
    );

  private inTable = (col: string): boolean =>
    this.writeColumns.length === 0 || this.writeColumns.includes(col);

  private reverseConvert = (item: T): Cells =>
    this.rowConverter.reverseConvert(this.mapper.reverseMap(item));

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
