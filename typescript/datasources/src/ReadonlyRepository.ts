import type { ICommand } from "./ICommand.ts";
import { executeReader } from "./ICommand.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IConnection } from "./IConnection.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { ArrayExpression } from "./statements/ArrayExpression.ts";
import { Identifier } from "./statements/Identifier.ts";
import { Literal } from "./statements/Literal.ts";
import { Operator } from "./statements/Operator.ts";
import { WhereClause } from "./statements/WhereClause.ts";
import { createRepositoryOptions } from "./createRepositoryOptions.ts";
import type { ReadonlyRepositoryOptions } from "./ReadonlyRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { parts } from "./RepositoryBuilder.ts";
import type { TypeConverter } from "./converters/TypeConverter.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { IdentityMapper } from "./mappers/IdentityMapper.ts";
import type { RepositoryStatements } from "./RepositoryStatements.ts";

export const untyped = <T>(): Constructable<T> => Object as never as Constructable<T>;

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
          this.renamed(
            reader.current ??
              (() => {
                throw new DatasourceError("Reader current must be a row");
              })(),
          ),
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
    where instanceof WhereClause ? new WhereClause(this.rewrite(where.condition, fieldsApplied)) : where;

  protected column = (property: string): string => this.columnMappings.map(property);

  protected cell = (property: string, value: Cell, fieldsApplied: boolean): Cell => {
    const typed = this.rowConverter instanceof RowTypeConverter ? this.rowConverter : undefined;
    const field = typed?.fields[property];
    if (field) return fieldsApplied ? value : field.reverseConvert(value);
    const kind =
      typeof value === "boolean"
        ? "bool"
        : value instanceof Date
          ? "datetime"
          : value instanceof Uint8Array
            ? "binary"
            : undefined;
    const converter = kind ? typed?.types[kind] : undefined;
    return converter ? converter.reverseConvert(value) : value;
  };

  private rewrite = (expression: IExpression, fieldsApplied: boolean): IExpression =>
    expression instanceof WhereClause
      ? this.rewrite(expression.condition, fieldsApplied)
      : expression instanceof Identifier
        ? new Identifier(this.columnMappings.map(expression.name))
        : expression instanceof Literal
          ? expression
          : expression instanceof ArrayExpression
            ? new ArrayExpression(expression.elements.map((element) => this.rewrite(element, fieldsApplied)))
            : expression instanceof Operator.Not
              ? new Operator.Not(this.rewrite(expression.argument, fieldsApplied))
              : expression instanceof Operator.And
                ? new Operator.And(
                    this.rewrite(expression.left, fieldsApplied),
                    this.rewrite(expression.right, fieldsApplied),
                  )
                : expression instanceof Operator.Or
                  ? new Operator.Or(
                      this.rewrite(expression.left, fieldsApplied),
                      this.rewrite(expression.right, fieldsApplied),
                    )
                  : expression instanceof Operator.In
                    ? new Operator.In(
                        this.rewrite(expression.left, fieldsApplied),
                        this.inRight(expression.left, expression.right, fieldsApplied),
                      )
                    : expression instanceof Operator.Equals
                      ? new Operator.Equals(
                          this.side(expression.left, expression.right, fieldsApplied),
                          this.side(expression.right, expression.left, fieldsApplied),
                        )
                      : expression instanceof Operator.NotEquals
                        ? new Operator.NotEquals(
                            this.side(expression.left, expression.right, fieldsApplied),
                            this.side(expression.right, expression.left, fieldsApplied),
                          )
                        : expression instanceof Operator.GreaterThan
                          ? new Operator.GreaterThan(
                              this.side(expression.left, expression.right, fieldsApplied),
                              this.side(expression.right, expression.left, fieldsApplied),
                            )
                          : expression instanceof Operator.LessThan
                            ? new Operator.LessThan(
                                this.side(expression.left, expression.right, fieldsApplied),
                                this.side(expression.right, expression.left, fieldsApplied),
                              )
                            : expression instanceof Operator.GreaterThanOrEqual
                              ? new Operator.GreaterThanOrEqual(
                                  this.side(expression.left, expression.right, fieldsApplied),
                                  this.side(expression.right, expression.left, fieldsApplied),
                                )
                              : expression instanceof Operator.LessThanOrEqual
                                ? new Operator.LessThanOrEqual(
                                    this.side(expression.left, expression.right, fieldsApplied),
                                    this.side(expression.right, expression.left, fieldsApplied),
                                  )
                                : (() => {
                                    throw new DatasourceError("empty expression");
                                  })();

  private side = (expression: IExpression, other: IExpression, fieldsApplied: boolean): IExpression =>
    expression instanceof Literal && other instanceof Identifier
      ? new Literal(this.cell(other.name, expression.value, fieldsApplied))
      : this.rewrite(expression, fieldsApplied);

  private inRight = (left: IExpression, right: IExpression, fieldsApplied: boolean): IExpression =>
    left instanceof Identifier && right instanceof ArrayExpression
      ? new ArrayExpression(
          right.elements.map((element) =>
            element instanceof Literal
              ? new Literal(this.cell(left.name, element.value, fieldsApplied))
              : this.rewrite(element, fieldsApplied),
          ),
        )
      : this.rewrite(right, fieldsApplied);
}
