import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { createRepositoryOptions } from "./createRepositoryOptions.ts";
import type { RepositoryOptions } from "./RepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

export const isBuilt = <T>(
  value: IConnection | BuiltRepository<T>,
): value is BuiltRepository<T> =>
  "statements" in value && "rowConverter" in value && "crudMapper" in value;

export const builtOf = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | RepositoryOptions<T>,
): BuiltRepository<T> | undefined =>
  typeof typeOrConnectionOrBuilt === "function"
    ? connectionOrOptions === undefined
      ? undefined
      : parts(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnectionOrBuilt))
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : parts(typeOrConnectionOrBuilt, connectionOrOptions as RepositoryOptions<T>);

export class ReadonlyRepository<T> implements IReadonlyRepository<T> {
  readonly type: Constructable<T>;
  protected connection?: IConnection;
  readonly rowConverter?: TypeConverter<T, Cells>;
  readonly mapper?: IMapper<T, T>;
  readonly statements?: RepositoryStatements;
  protected readonly middlewares: ICommandMiddleware[] = [];

  constructor(type: Constructable<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: RepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
    connectionOrOptions?: IConnection | RepositoryOptions<T>,
  ) {
    const built = builtOf(typeOrConnectionOrBuilt, connectionOrOptions);
    if (!built) {
      this.type = typeOrConnectionOrBuilt as Constructable<T>;
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
    const reader = await executeReader(
      this.command(),
      statement,
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
