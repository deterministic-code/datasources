import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import { repositoryParts } from "./CRUDRepositoryBuilder.ts";
import type { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { Response } from "./Response.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal, Operator, Where } from "./statements/index.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { ICommand } from "./ICommand.ts";
import type { Parameter } from "./Parameter.ts";
import type { BulkCommand } from "./BulkCommand.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { SingleCommandIBulkWriteStrategy } from "./SingleCommandIBulkWriteStrategy.ts";
import { UnitOfWorkTransaction } from "./UnitOfWorkTransaction.ts";

const fieldsOf = (
  record: Cells,
  include: (col: string) => boolean,
  cell: (property: string, value: Cell, fieldsApplied: boolean) => Cell,
  enrichmentInto: (name: string) => boolean,
): Field[] =>
  Object.entries(record)
    .filter(([key, value]) => include(key) && !enrichmentInto(key) && value !== null && value !== undefined)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: cell(name, value, true) as Scalar }));

const isBulkStrategy = <T extends object>(value: unknown): value is IBulkWriteStrategy<T> =>
  typeof value === "object" &&
  value !== null &&
  "create" in value &&
  "update" in value &&
  "delete" in value &&
  !("persist" in value) &&
  !("createCommand" in value) &&
  !("statements" in value);

const scalarOf = (value: Scalar | object | undefined): Scalar | undefined =>
  value === undefined || value === null || typeof value === "object" ? undefined : value;

const transactionOf = <T extends object>(
  options: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T> | undefined,
): boolean =>
  options !== undefined && "write" in options ? options.write?.useTransaction ?? false : false;

const inTransaction = async <TResult>(
  transactional: boolean,
  connection: IConnection,
  write: () => Promise<TResult>,
): Promise<TResult> => {
  if (!transactional) return write();
  const work = new UnitOfWorkTransaction(connection);
  await work.begin();
  try {
    const result = await write();
    await work.commit();
    return result;
  } catch (error) {
    await work.abort();
    throw error;
  }
};

export class BulkCRUDRepository<T extends object = Cells> extends CRUDRepository<T> {
  private readonly strategy: IBulkWriteStrategy<T>;
  private readonly transactional: boolean;
  private readonly crudStatements: CRUDRepositoryStatements;
  /** Widens the protected repository command so a bulk strategy can execute it. */
  declare command: () => ICommand;

  constructor(type: Constructable<T>, connection: IConnection, bulk?: IBulkWriteStrategy<T>);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T>,
    bulk?: IBulkWriteStrategy<T>,
  ) {
    const strategy = bulk ?? (isBulkStrategy<T>(connectionOrOptions) ? connectionOrOptions : undefined);
    const transactional = transactionOf(connectionOrOptions);
    const parent = (
      isBulkStrategy(connectionOrOptions) ? undefined : connectionOrOptions
    ) as IConnection & CRUDRepositoryOptions<T>;
    super(typeOrConnection as Constructable<T> & IConnection, parent);
    this.strategy = strategy ?? new SingleCommandIBulkWriteStrategy<T>();
    this.transactional = transactional;
    this.crudStatements = repositoryParts(typeOrConnection, parent).statements;
  }

  createBulk = (items: T[]): Promise<Response> =>
    inTransaction(this.transactional, this.command().connection, () => this.strategy.create(items, this));

  updateBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    inTransaction(this.transactional, this.command().connection, () => this.strategy.update(items, byWhere, this));

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    inTransaction(this.transactional, this.command().connection, () => this.strategy.delete(items, byWhere, this));

  createOne = async (item: T): Promise<Scalar | undefined> => this.idOf(await this.create(item));

  updateOne = (item: T, byWhere?: IWhere): Promise<Response> => this.update(item, byWhere);

  deleteOne = (item: T, byWhere?: IWhere): Promise<Response> => this.delete(item, byWhere);

  inserts = (items: T[]): BulkCommand[] => items.map((item) => this.insertCommand(item));

  updates = (items: T[], byWhere?: IWhere): BulkCommand[] =>
    items.map((item) => this.updateCommand(item, byWhere));

  deletes = (items: T[], byWhere?: IWhere): BulkCommand[] =>
    items.map((item) => this.deleteCommand(item, byWhere));

  rows = (items: T[]): Cells[] => items.map((item) => this.rowOf(item));

  private idOf = (item: T): Scalar | undefined =>
    this.ids.length === 1 ? scalarOf((item as Record<string, Scalar | object>)[this.ids[0]!]) : undefined;

  private insertCommand = (item: T): BulkCommand => {
    const statement = this.crudStatements.insert(
      fieldsOf(this.reverse(item), (col) => !this.ids.includes(col), this.cell, this.enrichmentInto),
    );
    return { text: statement.getText(), parameters: statement.fields };
  };

  private updateCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const fields = fieldsOf(record, (col) => !this.ids.includes(col), this.cell, this.enrichmentInto);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.crudStatements.update(
      fields,
      byWhere ?? this.idsWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    return { text: statement.getText(), parameters: statement.parameters };
  };

  private deleteCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const statement = this.crudStatements.delete(
      byWhere ?? this.idsWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    const listed = statement.where?.parameters ?? [];
    const parameters: Parameter[] = listed.length
      ? listed
      : statement.getText().includes("$")
        ? fieldsOf(record, () => true, this.cell, this.enrichmentInto)
        : [];
    return { text: statement.getText(), parameters };
  };

  private rowOf = (item: T): Cells => {
    const stored = this.reverse(item);
    const row: Cells = {};
    for (const name of this.crudStatements.readColumns) {
      const match = Object.entries(stored).find(([key]) => this.column(key) === name && !this.enrichmentInto(key));
      row[name] = match?.[1] ?? null;
    }
    return row;
  };

  private reverse = (item: T): Cells => {
    const row = this.objectConverter!.reverseConvert(item);
    return this.rowConverter ? this.rowConverter.reverseConvert(row) : row;
  };

  private idsWhere = (record: Cells): IWhere => {
    const parts: IExpression[] = this.ids.map((col) => {
      if (!Object.prototype.hasOwnProperty.call(record, col))
        throw new DatasourceError(`Missing Id value "${col}"`);
      return new Operator.Equals(col, record[col] as Scalar);
    });
    return new Where(
      parts.length === 0
        ? new Operator.Equals(new Literal(1), new Literal(1))
        : parts.reduce((left, right) => new Operator.And(left, right)),
    );
  };
}
