import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal } from "./statements/Literal.ts";
import { DefaultStatementTextRenderer, Operator, Where } from "./statements/index.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import { repositoryParts } from "./CRUDRepositoryBuilder.ts";
import type { CRUDRepositoryStatements } from "./CRUDRepositoryStatements.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
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
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";

type Props = Record<string, Scalar | object>;

const assigned = (value: Cell): boolean =>
  value !== null && value !== "" && value !== 0 && value !== 0n;

const listed = (list: readonly string[] | undefined, col: string, fallback: boolean): boolean =>
  list === undefined ? fallback : list.includes(col);

const writable = (
  key: string,
  columns: readonly string[],
  column: (property: string) => string,
): boolean =>
  columns.length === 0 || columns.includes(key) || columns.some((name) => column(name) === key);

const fieldsOf = (
  record: Cells,
  include: (col: string) => boolean,
  cell: (property: string, value: Cell, fieldsApplied: boolean) => Cell,
  enrichmentInto: (name: string) => boolean,
  columns: readonly string[],
  column: (property: string) => string,
): Field[] =>
  Object.entries(record)
    .filter(
      ([key, value]) =>
        include(key) &&
        writable(key, columns, column) &&
        !enrichmentInto(key) &&
        value !== null &&
        value !== undefined,
    )
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: cell(name, value, true) as Scalar }));

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


export class CRUDRepository<T = Cells>
  extends ReadonlyRepository<T>
  implements ICRUDRepository<T>
{
  readonly ids: readonly string[];
  private readonly createProperties?: readonly string[];
  private readonly updateProperties?: readonly string[];
  private readonly crudStatements: CRUDRepositoryStatements;
  private readonly tree: IRepositoryTree;
  private readonly write: IWriteEnrichmentStrategy<T>;

  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(
    typeOrConnection: Constructable<T> | IConnection,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
  ) {
    const options = connectionOrOptions as IConnection | CRUDRepositoryOptions<T>;
    const opened = repositoryParts(typeOrConnection, options);
    super(opened.built);
    this.ids = opened.built.ids;
    this.createProperties = (options as CRUDRepositoryOptions<T>).createProperties;
    this.updateProperties = (options as CRUDRepositoryOptions<T>).updateProperties;
    this.crudStatements = opened.statements;
    this.tree = new RepositoryTree(new RepositoryTreeNode(this, ""));
    this.write = (options as CRUDRepositoryOptions<T>).writeEnrichment ?? new DefaultWriteEnrichmentStrategy<T>();
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
    const stored = lastEnrichment(this) as StoredEnrichment<T & object>;
    stored.apply = (parents, sources, strategy) =>
      (strategy ?? chosen).applyLinked(via, link, enrichment, parents, sources);
    stored.link = link as ICRUDRepository<object>;
    return this;
  }

  create = (item: T, writeEnrichment?: IWriteEnrichmentStrategy<T>): Promise<T> =>
    this.inTransaction(() => this.createCore(item, this.writer(writeEnrichment)), this.writer(writeEnrichment));

  update = (item: T, byWhere?: IWhere, writeEnrichment?: IWriteEnrichmentStrategy<T>): Promise<Response> =>
    this.inTransaction(() => this.updateCore(item, byWhere, this.writer(writeEnrichment)), this.writer(writeEnrichment));

  delete = (item: T, byWhere?: IWhere, writeEnrichment?: IWriteEnrichmentStrategy<T>): Promise<Response> =>
    this.inTransaction(() => this.deleteCore(item, byWhere, this.writer(writeEnrichment)), this.writer(writeEnrichment));

  private writer = (writeEnrichment?: IWriteEnrichmentStrategy<T>): IWriteEnrichmentStrategy<T> =>
    writeEnrichment ?? this.write;

  private inTransaction = async <TResult>(
    write: () => Promise<TResult>,
    enrichment: IWriteEnrichmentStrategy<T>,
  ): Promise<TResult> => {
    if (!enrichment.useTransaction) return write();
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

  private createCore = async (item: T, write: IWriteEnrichmentStrategy<T>): Promise<T> => {
    await this.insert(item);
    if (!this.enriched) return item;
    await write.persist(this.tree, [item], "create");
    if (this.ids.length !== 1) return item;
    const id = this.idValue(item);
    return id === undefined ? item : (await this.readSingleById(id)) ?? item;
  };

  private updateCore = async (
    item: T,
    byWhere: IWhere | undefined,
    write: IWriteEnrichmentStrategy<T>,
  ): Promise<Response> => {
    const response = await this.save(item, byWhere);
    if (this.enriched) await write.persist(this.tree, [item], "update");
    return response;
  };

  private deleteCore = async (
    item: T,
    byWhere: IWhere | undefined,
    write: IWriteEnrichmentStrategy<T>,
  ): Promise<Response> => {
    if (this.enriched) await write.persist(this.tree, [item], "delete");
    return this.remove(item, byWhere);
  };

  readSingleById = async (id: Scalar | Cells): Promise<T | undefined> =>
    this.readSingle(this.byId(id));

  private insert = async (item: T): Promise<Response> => {
    const record = this.reverseConvert(item);
    const fields = fieldsOf(
      record,
      (col) => listed(this.createProperties, col, !this.ids.includes(col) || assigned(record[col] ?? null)),
      this.cell,
      this.enrichmentInto,
      this.crudStatements.readColumns,
      this.column,
    );
    const statement = this.crudStatements.insert(fields);
    const response = await execute(this.command(), statement, statement.fields);
    if (this.ids.length === 1 && !fields.some((field) => field.name === this.ids[0]))
      await this.copyInsertedId(item, fields);
    return response;
  };

  private copyInsertedId = async (target: T, fields: Field[]): Promise<void> => {
    void fields;
    const column = this.ids[0]!;
    const statement = this.crudStatements.selectInsertedId(column);
    const sqlColumn = statement.column;
    const reader = await executeReader(this.command(), statement);
    try {
      if (!(await reader.next())) return;
      const current = reader.current;
      if (!current) throw new DatasourceError("Reader current must be a row");
      const row = identified(named(current, column, sqlColumn), column);
      const cells = this.rowConverter ? this.rowConverter.convert(row) : row;
      const converted =
        this.objectConverter instanceof MetaDataRowTypeConverter
          ? this.objectConverter.convert(cells, this.type)
          : this.objectConverter!.convert(cells);
      const value = scalarOf((converted as Props)[column]);
      if (value !== undefined) (target as Props)[column] = value;
    } finally {
      await reader.dispose();
    }
  };

  private idValue = (item: T): Scalar | undefined => scalarOf((item as Props)[this.ids[0]!]);

  private save = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const fields = fieldsOf(
      record,
      (col) => listed(this.updateProperties, col, !this.ids.includes(col)),
      this.cell,
      this.enrichmentInto,
      this.crudStatements.readColumns,
      this.column,
    );
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.crudStatements.update(
      fields,
      byWhere ?? this.idWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    return execute(this.command(), statement, statement.parameters);
  };

  private remove = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const statement = this.crudStatements.delete(
      byWhere ?? this.idWhere(record),
      (name, value) => this.cell(name, value, byWhere === undefined),
    );
    const listed = statement.where?.parameters ?? [];
    const parameters = listed.length
      ? listed
      : DefaultStatementTextRenderer.instance.render(statement).includes("$")
        ? fieldsOf(record, () => true, this.cell, this.enrichmentInto, this.crudStatements.readColumns, this.column)
        : [];
    return execute(this.command(), statement, parameters);
  };

  private byId = (id: Scalar | Cells): IWhere => {
    if (this.ids.length === 0) throw new DatasourceError("ReadSingleById requires an Id column");
    if (isCells(id)) return this.idWhere(id);
    if (this.ids.length !== 1)
      throw new DatasourceError("Composite id requires a row keyed by each Id column");
    return this.idWhere({ [this.ids[0]!]: id });
  };

  private reverseConvert = (item: T): Cells => {
    const row = this.objectConverter!.reverseConvert(item);
    return this.rowConverter ? this.rowConverter.reverseConvert(row) : row;
  };

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
    enrichmentsOf(this)!.some((entry) => entry.enrichment.destinationProperty === name);
}

const writerOf = <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
): ICRUDRepository<object> | undefined =>
  "create" in via && typeof via.create === "function" ? (via as ICRUDRepository<object>) : undefined;

const lastEnrichment = (host: object): StoredEnrichment => enrichmentsOf(host)!.at(-1)!;
