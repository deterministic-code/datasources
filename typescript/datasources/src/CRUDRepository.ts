import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal } from "./statements/Literal.ts";
import { Operator, Where } from "./statements/index.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { createRepositoryOptions, defaultId, fieldNamesOf } from "./createRepositoryOptions.ts";
import { identityCreate, parts } from "./RepositoryBuilder.ts";
import { isBuilt, ReadonlyRepository } from "./ReadonlyRepository.ts";
import { execute } from "./ICommand.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultWriteEnrichmentStrategy } from "./DefaultWriteEnrichmentStrategy.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";

type Props = Record<string, Scalar | object>;

const fieldsOf = (
  record: Cells,
  include: (col: string) => boolean,
  column: (property: string) => string,
  cell: (property: string, value: Cell, fieldsApplied: boolean) => Cell,
): Field[] =>
  Object.entries(record)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name: column(name), value: cell(name, value, true) as Scalar }));

const apply = <T>(item: T, identity: T): void => {
  if (!item || !identity || typeof item !== "object" || typeof identity !== "object") return;
  for (const [key, value] of Object.entries(identity))
    if (value != null) (item as Record<string, Scalar>)[key] = value as Scalar;
};

const asScalar = (value: Scalar | undefined): Scalar | undefined =>
  value == null ? undefined : value;

const isCells = (id: Scalar | Cells): id is Cells => typeof id === "object" && id !== null;

const scalarOf = (value: Scalar | object | undefined): Scalar | undefined =>
  value === undefined || value === null || typeof value === "object" ? undefined : value;

const copyIds = (item: object, created: object, names: readonly string[]): void => {
  if (item === created) return;
  const source = created as Props;
  const dest = item as Props;
  for (const name of names) {
    const value = scalarOf(source[name]);
    if (value !== undefined) dest[name] = value;
  }
};

const crudBuilt = <T>(
  typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T>,
  connectionOrOptions?: IConnection | CRUDRepositoryOptions<T>,
): BuiltRepository<T> =>
  typeof typeOrConnectionOrBuilt === "function"
    ? parts(connectionOrOptions as IConnection, createRepositoryOptions(typeOrConnectionOrBuilt))
    : isBuilt(typeOrConnectionOrBuilt)
      ? typeOrConnectionOrBuilt
      : parts(typeOrConnectionOrBuilt, withIds(connectionOrOptions as CRUDRepositoryOptions<T>));

const withIds = <T>(options: CRUDRepositoryOptions<T>): CRUDRepositoryOptions<T> =>
  "ids" in options
    ? options
    : {
        ...options,
        ids: options.type ? defaultId(fieldNamesOf(options.type)) : ["id"],
      };

const isDelegate = <T>(
  value: Constructable<T> | IConnection | BuiltRepository<T> | ICRUDRepository<T>,
): value is ICRUDRepository<T> =>
  typeof value === "object" &&
  value !== null &&
  "readSingleById" in value &&
  typeof value.create === "function" &&
  !("createCommand" in value);

export class CRUDRepository<T = Cells>
  extends ReadonlyRepository<T>
  implements ICRUDRepository<T>
{
  readonly ids: readonly string[];
  private readonly createMapper?: IMapper<Scalar, T>;
  private readonly write: IWriteEnrichmentStrategy<T>;

  private get stored(): ICRUDRepository<T> | undefined {
    return this.innerRepo && "readSingleById" in this.innerRepo
      ? (this.innerRepo as ICRUDRepository<T>)
      : undefined;
  }

  constructor(
    inner: ICRUDRepository<T>,
    strategy?: IReadonlyEnrichmentStrategy<T>,
    write?: IWriteEnrichmentStrategy<T>,
  );
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T> | ICRUDRepository<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IReadonlyEnrichmentStrategy<T>,
    write?: IWriteEnrichmentStrategy<T>,
  ) {
    if (isDelegate(typeOrConnectionOrBuilt)) {
      super(typeOrConnectionOrBuilt, connectionOrOptions as IReadonlyEnrichmentStrategy<T> | undefined);
      this.write = write ?? new DefaultWriteEnrichmentStrategy<T>();
      this.ids = typeOrConnectionOrBuilt.ids;
      this.createMapper =
        typeOrConnectionOrBuilt instanceof CRUDRepository && typeOrConnectionOrBuilt.createMapper
          ? typeOrConnectionOrBuilt.createMapper
          : identityCreate<T>(this.ids);
      return;
    }
    const built = crudBuilt(typeOrConnectionOrBuilt, connectionOrOptions as IConnection | CRUDRepositoryOptions<T>);
    super(built);
    this.createMapper = built.create;
    this.ids = built.ids;
    this.write = new DefaultWriteEnrichmentStrategy<T>();
  }

  create = async (item: T): Promise<T> => {
    const stored = this.enriched ? this.stripEnriched(item) : item;
    const created = this.stored ? await this.stored.create(stored) : await this.inserted(stored);
    if (!this.enriched) return created;
    const names = this.ids.length > 0 ? this.ids : defaultId(Object.keys(item as object));
    copyIds(item, created, names);
    await this.write.persist(this.tree, [item], "create");
    const id = names.length === 1 ? scalarOf((item as Props)[names[0]!]) : undefined;
    return id === undefined ? item : (await this.readSingleById(id)) ?? item;
  };

  update = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const stored = this.enriched ? this.stripEnriched(item) : item;
    const response = this.stored
      ? await this.stored.update(stored, byWhere)
      : await this.save(stored, byWhere);
    if (this.enriched) await this.write.persist(this.tree, [item], "update");
    return response;
  };

  delete = async (item: T, byWhere?: IWhere): Promise<Response> => {
    if (this.enriched) await this.write.persist(this.tree, [item], "delete");
    return this.stored ? this.stored.delete(item, byWhere) : this.remove(item, byWhere);
  };

  readSingleById = async (id: Scalar | Cells): Promise<T | undefined> => {
    if (!this.stored) return this.readSingle(this.byId(id));
    const item = await this.stored.readSingleById(id);
    if (!item || !this.enriched) return item;
    await this.enrich([item]);
    return item;
  };

  private inserted = async (item: T): Promise<T> => {
    await this.insert(item);
    return item;
  };

  private insert = async (item: T): Promise<Response> => {
    const fields = fieldsOf(this.reverseConvert(item), (col) => !this.ids.includes(col), this.column, this.cell);
    const response = await execute(this.command(), this.statements!.insert(fields), fields);
    const scalar = asScalar(response.result as string | number | boolean | bigint | null | undefined);
    if (scalar !== undefined && this.createMapper) apply(item, this.createMapper.map(scalar));
    return response;
  };

  private save = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.stored(byWhere) : this.stored(this.idWhere(record), true);
    const fields = fieldsOf(record, (col) => !this.ids.includes(col), this.column, this.cell);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    const statement = this.statements!.update(fields, where);
    return this.command().execute(statement.getText(), statement.parameters);
  };

  private remove = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.stored(byWhere) : this.stored(this.idWhere(record), true);
    const statement = this.statements!.delete(where);
    const listed = where.parameters ?? [];
    return execute(
      this.command(),
      statement,
      listed.length ? listed : statement.getText().includes("$")
        ? fieldsOf(record, () => true, this.column, this.cell)
        : [],
    );
  };

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

  private stripEnriched = (item: T): T => this.stripNode(this.tree.parent, item) as T;

  private stripNode = (node: IRepositoryTreeNode, item: object): object => {
    const copy = { ...item } as Props;
    for (const child of node.items)
      if (child.enrichment) delete copy[child.enrichment.into];
    return copy;
  };
}
