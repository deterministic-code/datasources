import type { ICommand } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { createRepositoryBuilderOptions } from "./createRepositoryBuilderOptions.ts";
import type { RepositoryBuilderOptions } from "./RepositoryBuilderOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export const isBuilt = <T>(
  value: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  typeof value === "object" &&
  value !== null &&
  "statements" in value &&
  "rowConverter" in value &&
  "crudMapper" in value;

export const builtOf = <T>(
  typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
  connection?: IConnection,
): BuiltRepository<T> | undefined =>
  typeof typeOrOptions === "function" && connection === undefined
    ? undefined
    : isBuilt(typeOrOptions)
      ? typeOrOptions
      : parts(
          connection === undefined
            ? (typeOrOptions as RepositoryBuilderOptions<T>)
            : createRepositoryBuilderOptions(typeOrOptions as Constructable<T>, connection),
        );

export class ReadonlyRepository<T> implements IReadonlyRepository<T> {
  readonly type: Constructable<T>;
  protected connection?: IConnection;
  readonly rowConverter?: TypeConverter<T, Cells>;
  readonly mapper?: IMapper<T, T>;
  readonly statements?: RepositoryStatements;
  protected readonly middlewares: ICommandMiddleware[] = [];

  constructor(type: Constructable<T>);
  constructor(options: RepositoryBuilderOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(
    typeOrOptions: Constructable<T> | RepositoryBuilderOptions<T> | BuiltRepository<T>,
    connection?: IConnection,
  ) {
    const built = builtOf(typeOrOptions, connection);
    if (!built) {
      this.type = typeOrOptions as Constructable<T>;
      return;
    }
    this.type = built.type;
    this.connection = built.connection;
    this.rowConverter = built.rowConverter;
    this.mapper = built.crudMapper.rows;
    this.statements = built.statements;
  }

  addMiddleware = (middleware: ICommandMiddleware): this => {
    this.middlewares.push(middleware);
    return this;
  };

  async *read(query?: IQuery | IWhere): AsyncIterable<T> {
    const statement = this.readQuery(query);
    const reader = await this.command().executeReader(
      statement.getText(),
      statement.where?.parameters ?? statement.parameters,
    );
    while (await reader.next())
      yield this.mapper!.map(
        this.rowConverter!.convert(
          reader.current ??
            (() => {
              throw new DatasourceError("Reader current must be a row");
            })(),
        ),
      );
  }

  async readSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    for await (const item of this.read(query)) return item;
    return undefined;
  }

  protected command = (): ICommand =>
    this.middlewares.reduce(
      (command, middleware) => command.addMiddleware(middleware),
      this.connection!.createCommand(),
    );

  protected readQuery = (query?: IQuery | IWhere): IQuery =>
    !query ? this.statements!.query() : "where" in query ? query : this.statements!.query(query);
}
