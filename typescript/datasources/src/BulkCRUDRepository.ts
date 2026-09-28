import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { Response } from "./Response.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal, Operator, UpdateStatement, Where } from "./statements/index.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { ICommand } from "./ICommand.ts";
import type { Parameter } from "./Parameter.ts";
import type { BulkCommand } from "./BulkCommand.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { SingleCommands } from "./SingleCommands.ts";
import { UnitOfWorkTransaction } from "./UnitOfWorkTransaction.ts";

const fieldsOf = (
  record: Cells,
  include: (col: string) => boolean,
  column: (property: string) => string,
  cell: (property: string, value: Cell, fieldsApplied: boolean) => Cell,
): Field[] =>
  Object.entries(record)
    .filter(([key, value]) => include(key) && value !== null && value !== undefined)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name: column(name), value: cell(name, value, true) as Scalar }));

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
  /** Widens the protected repository command so a bulk strategy can execute it. */
  declare command: () => ICommand;

  constructor(type: Constructable<T>, connection: IConnection, bulk?: IBulkWriteStrategy<T>);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(built: BuiltRepository<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(
    typeOrConnectionOrBuilt:
      | Constructable<T>
      | IConnection
      | BuiltRepository<T>
      | IBulkWriteStrategy<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T>,
    bulk?: IBulkWriteStrategy<T>,
  ) {
    const strategy = bulk ?? (isBulkStrategy<T>(connectionOrOptions) ? connectionOrOptions : undefined);
    const transactional = transactionOf(connectionOrOptions);
    super(
      typeOrConnectionOrBuilt as Constructable<T> & IConnection & BuiltRepository<T>,
      (isBulkStrategy(connectionOrOptions) ? undefined : connectionOrOptions) as IConnection & CRUDRepositoryOptions<T>,
    );
    this.strategy = strategy ?? new SingleCommands<T>();
    this.transactional = transactional;
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

  inserts = (items: T[]): BulkCommand[] => items.map((item) => this.insertCommand(this.stripped(item)));

  updates = (items: T[], byWhere?: IWhere): BulkCommand[] =>
    items.map((item) => this.updateCommand(this.stripped(item), byWhere));

  deletes = (items: T[], byWhere?: IWhere): BulkCommand[] =>
    items.map((item) => this.deleteCommand(this.stripped(item), byWhere));

  rows = (items: T[]): Cells[] => items.map((item) => this.rowOf(item));

  private idOf = (item: T): Scalar | undefined =>
    this.ids.length === 1 ? scalarOf((item as Record<string, Scalar | object>)[this.ids[0]!]) : undefined;

  private stripped = (item: T): T => (this.enriched ? this.stripEnriched(item) : item);

  private insertCommand = (stored: T): BulkCommand => {
    const fields = fieldsOf(this.reverse(stored), (col) => !this.ids.includes(col), this.column, this.cell);
    return { text: this.statements!.insert(fields).getText(), parameters: fields };
  };

  private updateCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const where = byWhere ? this.map(byWhere, false) : this.map(this.idsWhere(record), true);
    const fields = fieldsOf(record, (col) => !this.ids.includes(col), this.column, this.cell);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    return { text: this.statements!.update(fields, where).getText(), parameters: UpdateStatement.parameterList(fields, where) };
  };

  private deleteCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const where = byWhere ? this.map(byWhere, false) : this.map(this.idsWhere(record), true);
    const statement = this.statements!.delete(where);
    const listed = where.parameters ?? [];
    const parameters: Parameter[] = listed.length
      ? listed
      : statement.getText().includes("$")
        ? fieldsOf(record, () => true, this.column, this.cell)
        : [];
    return { text: statement.getText(), parameters };
  };

  private rowOf = (item: T): Cells => {
    const stored = this.reverse(this.stripped(item));
    const row: Cells = {};
    for (const name of this.statements?.columns ?? []) {
      const match = Object.entries(stored).find(([key]) => this.column(key) === name);
      row[name] = match?.[1] ?? null;
    }
    return row;
  };

  private reverse = (item: T): Cells => this.rowConverter!.reverseConvert(this.mapper!.reverseMap(item));

  private map = (where: IWhere, fieldsApplied: boolean): IWhere =>
    where instanceof Where ? where.map(this.column, (name, cell) => this.cell(name, cell, fieldsApplied)) : where;

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
