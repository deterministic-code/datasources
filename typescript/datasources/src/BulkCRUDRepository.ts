import type { IConnection } from "./IConnection.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { Response } from "./Response.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal, Operator, UpdateStatement, Where } from "./statements/index.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Parameter } from "./Parameter.ts";
import type { BulkCommand } from "./BulkCommand.ts";
import type { IBulkWrite } from "./IBulkWrite.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { strip } from "./DefaultWriteEnrichmentStrategy.ts";
import { SingleCommands } from "./SingleCommands.ts";
import { UnitOfWork } from "./UnitOfWork.ts";

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

const isBulkStrategy = <T>(value: unknown): value is IBulkWriteStrategy<T> =>
  typeof value === "object" &&
  value !== null &&
  "create" in value &&
  "update" in value &&
  "delete" in value &&
  !("persist" in value) &&
  !("createCommand" in value) &&
  !("statements" in value);

export class BulkCRUDRepository<T extends object = Cells> extends CRUDRepository<T> {
  private readonly strategy: IBulkWriteStrategy<T>;

  constructor(type: Constructable<T>, connection: IConnection, bulk?: IBulkWriteStrategy<T>);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(built: BuiltRepository<T>, bulk?: IBulkWriteStrategy<T>);
  constructor(write?: IWriteEnrichmentStrategy<T>);
  constructor(
    typeOrConnectionOrBuiltOrWrite?:
      | Constructable<T>
      | IConnection
      | BuiltRepository<T>
      | IWriteEnrichmentStrategy<T>
      | IBulkWriteStrategy<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IBulkWriteStrategy<T>,
    bulk?: IBulkWriteStrategy<T>,
  ) {
    const strategy = bulk ?? (isBulkStrategy<T>(connectionOrOptions) ? connectionOrOptions : undefined);
    super(
      typeOrConnectionOrBuiltOrWrite as Constructable<T> & IConnection & BuiltRepository<T> & IWriteEnrichmentStrategy<T>,
      (isBulkStrategy(connectionOrOptions) ? undefined : connectionOrOptions) as IConnection & CRUDRepositoryOptions<T>,
    );
    this.strategy = strategy ?? new SingleCommands<T>();
  }

  createMany = (items: T[]): Promise<Response> =>
    this.useTransaction
      ? new UnitOfWork(this.command().connection).inTransaction(() => this.strategy.create(items, this.asWrite()))
      : this.strategy.create(items, this.asWrite());

  updateMany = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.useTransaction
      ? new UnitOfWork(this.command().connection).inTransaction(() => this.strategy.update(items, byWhere, this.asWrite()))
      : this.strategy.update(items, byWhere, this.asWrite());

  deleteMany = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.useTransaction
      ? new UnitOfWork(this.command().connection).inTransaction(() => this.strategy.delete(items, byWhere, this.asWrite()))
      : this.strategy.delete(items, byWhere, this.asWrite());

  createBulk = (items: T[]): Promise<Response> => this.createMany(items);

  updateBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.updateMany(items, byWhere);

  deleteBulk = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.deleteMany(items, byWhere);

  bulkCreate = (items: T[]): Promise<Response> => this.createMany(items);

  bulkUpdate = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.updateMany(items, byWhere);

  bulkDelete = (items: T[], byWhere?: IWhere): Promise<Response> =>
    this.deleteMany(items, byWhere);

  private asWrite = (): IBulkWrite<T> => ({
    command: this.command(),
    createOne: async (item) => this.idValue(await this.createCore(item)),
    updateOne: (item, byWhere) => this.updateCore(item, byWhere),
    deleteOne: (item, byWhere) => this.deleteCore(item, byWhere),
    inserts: (items) => items.map((item) => this.insertCommand(this.stored(item))),
    updates: (items, byWhere) => items.map((item) => this.updateCommand(this.stored(item), byWhere)),
    deletes: (items, byWhere) => items.map((item) => this.deleteCommand(this.stored(item), byWhere)),
    rows: (items) => items.map((item) => this.rowOf(item)),
  });

  private stored = (item: T): T => (this.enriched ? (strip(this, item) as T) : item);

  private insertCommand = (stored: T): BulkCommand => {
    const fields = fieldsOf(this.reverse(stored), (col) => !this.ids.includes(col), this.column, this.cell);
    return { text: this.statements!.insert(fields).getText(), parameters: fields };
  };

  private updateCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const where = byWhere ? this.map(byWhere, false) : this.map(this.idWhere(record), true);
    const fields = fieldsOf(record, (col) => !this.ids.includes(col), this.column, this.cell);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    return { text: this.statements!.update(fields, where).getText(), parameters: UpdateStatement.parameterList(fields, where) };
  };

  private deleteCommand = (item: T, byWhere?: IWhere): BulkCommand => {
    const record = this.reverse(item);
    const where = byWhere ? this.map(byWhere, false) : this.map(this.idWhere(record), true);
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
    const stored = this.reverse(this.stored(item));
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

  private idWhere = (record: Cells): IWhere => {
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
