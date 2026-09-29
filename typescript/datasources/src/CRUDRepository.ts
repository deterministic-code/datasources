import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import type { IReturnStatement } from "./statements/IReturnStatement.ts";
import { Literal } from "./statements/Literal.ts";
import { Operator, ReturnStatement, UpdateStatement, Where } from "./statements/index.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import { createCRUDRepositoryOptions, defaultId, fieldNamesOf } from "./createCRUDRepositoryOptions.ts";
import { parts } from "./RepositoryBuilder.ts";
import { RowTypeConverter } from "./converters/RowTypeConverter.ts";
import { enrichmentsOf, ReadonlyRepository } from "./ReadonlyRepository.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import { StoredEnrichment } from "./StoredEnrichment.ts";
import { execute, executeReader } from "./ICommand.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { UnitOfWorkTransaction } from "./UnitOfWorkTransaction.ts";

type Props = Record<string, Scalar | object>;

const fieldsOf = (
  record: Cells,
  include: (col: string) => boolean,
  column: (property: string) => string,
  cell: (property: string, value: Cell, fieldsApplied: boolean) => Cell,
  enrichmentInto: (name: string) => boolean,
): Field[] =>
  Object.entries(record)
    .filter(([key, value]) => include(key) && !enrichmentInto(key) && value !== null && value !== undefined)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name: column(name), value: cell(name, value, true) as Scalar }));

const isCells = (id: Scalar | Cells): id is Cells => typeof id === "object" && id !== null;

const named = (row: Cells, property: string, sqlColumn: string): Cells => {
  if (property === sqlColumn || !(sqlColumn in row)) return row;
  const { [sqlColumn]: cell, ...rest } = row;
  return { ...rest, [property]: cell };
};

const identified = (row: Cells, column: string): Cells => {
  const values = Object.values(row);
  return column in row || values.length !== 1 ? row : { [column]: values[0] };
};

const scalarOf = (value: Scalar | object | undefined): Scalar | undefined =>
  value === undefined || value === null || typeof value === "object" ? undefined : value;

const crudBuilt = <T>(
  typeOrConnection: Constructable<T> | IConnection,
  connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
): BuiltRepository<T> =>
  typeof typeOrConnection === "function"
    ? parts(connectionOrOptions as IConnection, createCRUDRepositoryOptions(typeOrConnection))
    : parts(typeOrConnection, withIds(connectionOrOptions as CRUDRepositoryOptions<T>));

const withIds = <T>(options: CRUDRepositoryOptions<T>): CRUDRepositoryOptions<T> =>
  "ids" in options
    ? options
    : {
        ...options,
        ids: options.type ? defaultId(fieldNamesOf(options.type)) : ["id"],
      };

export class CRUDRepository<T = Cells>
  extends ReadonlyRepository<T>
  implements ICRUDRepository<T>
{
  readonly ids: readonly string[];
  private readonly write: IWriteEnrichmentStrategy<T>;
  private readonly returnMapper?: IMapper<IReturnStatement, IReturnStatement>;
  private get useTransaction(): boolean {
    return this.write.useTransaction;
  }

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
  ) {
    const options = connectionOrOptions as IConnection | CRUDRepositoryOptions<T>;
    const built = crudBuilt(typeOrConnection, options);
    super(built);
    this.ids = built.ids;
    const crud = typeof options === "object" && options !== null ? (options as CRUDRepositoryOptions<T>) : undefined;
    this.write = crud?.write ?? new DefaultWriteEnrichmentStrategy<T>();
    this.returnMapper = crud?.return;
  }

  override addEnriched<TRelated extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this {
    super.addEnriched(enrichment, via, strategy);
    const writer = writerOf(via);
    if (writer) lastEnrichment(this).persist = writer;
    return this;
  }

  addLinked<TRelated extends object, TLink extends object>(
    enrichment: Enrichment,
    via: IReadonlyRepository<TRelated>,
    link: ICRUDRepository<TLink>,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this {
    const chosen = strategy ?? new DefaultReadonlyEnrichmentStrategy<T & object>();
    super.addEnriched(enrichment, via, strategy);
    const stored = lastEnrichment(this);
    stored.apply = (parents, sources) =>
      chosen.applyLinked(via, link, enrichment, parents, sources);
    stored.link = link as ICRUDRepository<object>;
    return this;
  }

  create = (item: T): Promise<T> => this.inTransaction(() => this.createCore(item));

  update = (item: T, byWhere?: IWhere): Promise<Response> =>
    this.inTransaction(() => this.updateCore(item, byWhere));

  delete = (item: T, byWhere?: IWhere): Promise<Response> =>
    this.inTransaction(() => this.deleteCore(item, byWhere));

  private inTransaction = async <TResult>(write: () => Promise<TResult>): Promise<TResult> => {
    if (!this.useTransaction) return write();
    const work = new UnitOfWorkTransaction(this.command().connection);
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

  private createCore = async (item: T): Promise<T> => {
    await this.insert(item);
    if (!this.enriched) return item;
    await this.write.persist(this.tree, [item], "create");
    if (this.ids.length !== 1) return item;
    const id = this.idValue(item);
    return id === undefined ? item : (await this.readSingleById(id)) ?? item;
  };

  private updateCore = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const response = await this.save(item, byWhere);
    if (this.enriched) await this.write.persist(this.tree, [item], "update");
    return response;
  };

  private deleteCore = async (item: T, byWhere?: IWhere): Promise<Response> => {
    if (this.enriched) await this.write.persist(this.tree, [item], "delete");
    return this.remove(item, byWhere);
  };

  readSingleById = async (id: Scalar | Cells): Promise<T | undefined> =>
    this.readSingle(this.byId(id));

  private insert = async (item: T): Promise<Response> => {
    const fields = fieldsOf(
      this.reverseConvert(item),
      (col) => !this.ids.includes(col),
      this.column,
      this.cell,
      this.enrichmentInto,
    );
    const response = await execute(this.command(), this.statements!.insert(fields), fields);
    if (this.ids.length === 1) await this.copyInsertedId(item, fields);
    return response;
  };

  private copyInsertedId = async (target: T, fields: Field[]): Promise<void> => {
    const column = this.ids[0]!;
    const sqlColumn = this.column(column);
    const statement = this.returnMapper
      ? this.returnMapper.map(new ReturnStatement(sqlColumn, fields))
      : this.statements!.selectInsertedId(sqlColumn);
    const reader = await executeReader(
      this.command(),
      statement,
      statement instanceof ReturnStatement ? fields : undefined,
    );
    try {
      if (!(await reader.next())) return;
      const current = reader.current;
      if (!current) throw new DatasourceError("Reader current must be a row");
      const row = identified(named(current, column, sqlColumn), column);
      const converted = (
        this.rowConverter instanceof RowTypeConverter
          ? this.rowConverter.convert(row, this.type)
          : this.rowConverter!.convert(row)
      ) as T;
      this.writeId(target, this.idValue(this.mapper!.map(converted)));
    } finally {
      await reader.dispose();
    }
  };

  private writeId = (item: T, id: Scalar | undefined): void => {
    const scalar = scalarOf(id);
    if (this.ids.length !== 1 || scalar === undefined || !this.rowConverter) return;
    const name = this.ids[0]!;
    const patch = (
      this.rowConverter instanceof RowTypeConverter
        ? this.rowConverter.convert({ [name]: scalar }, this.type)
        : this.rowConverter.convert({ [name]: scalar })
    ) as Props;
    const value = patch[name];
    if (value !== undefined && value !== null && typeof value !== "object")
      (item as Props)[name] = value;
  };

  private idValue = (item: T): Scalar | undefined =>
    this.ids.length === 1 ? scalarOf((item as Props)[this.ids[0]!]) : undefined;

  private save = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.mapped(byWhere, false) : this.mapped(this.idWhere(record), true);
    const fields = fieldsOf(record, (col) => !this.ids.includes(col), this.column, this.cell, this.enrichmentInto);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    return execute(this.command(), this.statements!.update(fields, where), UpdateStatement.parameterList(fields, where));
  };

  private remove = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.mapped(byWhere, false) : this.mapped(this.idWhere(record), true);
    const statement = this.statements!.delete(where);
    const listed = where.parameters ?? [];
    const parameters = listed.length
      ? listed
      : statement.getText().includes("$")
        ? fieldsOf(record, () => true, this.column, this.cell, this.enrichmentInto)
        : [];
    return execute(this.command(), statement, parameters);
  };

  private mapped = (where: IWhere, fieldsApplied: boolean): IWhere =>
    where instanceof Where
      ? where.map(this.column, (name, cell) => this.cell(name, cell, fieldsApplied))
      : where;

  private byId = (id: Scalar | Cells): IWhere => {
    if (this.ids.length === 0) throw new DatasourceError("ReadSingleById requires an Id column");
    if (isCells(id)) return this.idWhere(id);
    if (this.ids.length !== 1)
      throw new DatasourceError("Composite id requires a row keyed by each Id column");
    return this.idWhere({ [this.ids[0]!]: id });
  };

  private reverseConvert = (item: T): Cells =>
    this.rowConverter!.reverseConvert(this.mapper!.reverseMap(item));

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

  protected enrichmentInto = (name: string): boolean =>
    enrichmentsOf(this)?.some((entry) => entry.enrichment.into === name) ?? false;
}

const writerOf = <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
): ICRUDRepository<object> | undefined =>
  "create" in via && typeof via.create === "function" ? (via as ICRUDRepository<object>) : undefined;

const lastEnrichment = (host: object): StoredEnrichment => {
  const entries = enrichmentsOf(host);
  const stored = entries?.[entries.length - 1];
  if (!stored) throw new DatasourceError("Missing enrichment");
  return stored;
};
