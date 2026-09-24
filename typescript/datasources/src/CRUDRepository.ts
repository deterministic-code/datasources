import type { IConnection } from "./IConnection.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./IWhere.ts";
import type { Field } from "./builders/Field.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { IExpression } from "./statements/IExpression.ts";
import { Literal } from "./statements/Literal.ts";
import { Operator, WhereClause } from "./statements/index.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { Constructable } from "./converters/Constructable.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { BuiltRepository } from "./BuiltRepository.ts";
import type { IMapper } from "./mappers/IMapper.ts";
import { createRepositoryOptions, defaultId, fieldNamesOf } from "./createRepositoryOptions.ts";
import { identityCreate, parts } from "./RepositoryBuilder.ts";
import { isBuilt, ReadonlyRepository, readTracked, tracked, type Sourced } from "./ReadonlyRepository.ts";
import { execute } from "./ICommand.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { IEnriched } from "./IEnriched.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IExecuteStoredProcedureStatement } from "./statements/IExecuteStoredProcedureStatement.ts";
import type { IQuery } from "./statements/IQuery.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import { RepositoryTree } from "./RepositoryTree.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { Row } from "./Row.ts";

type Props = Record<string, Scalar | object>;
type Mode = "create" | "update" | "delete";
type ReadQuery = IQuery | IWhere | IExecuteStoredProcedureStatement;

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

const props = (item: object): Props => item as Props;

const isWriter = (via: object): via is ICRUDRepository<object> =>
  "create" in via && typeof via.create === "function";

const writerOf = (via: object, into: string): ICRUDRepository<object> =>
  isWriter(via)
    ? via
    : (() => {
        throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
      })();

const get = (item: object, name: string): Scalar | object | undefined => props(item)[name];

const set = (item: object, name: string, value: Scalar): void => {
  Object.assign(item, { [name]: value });
};

const present = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

const hasKey = (item: object, keys: readonly string[]): boolean =>
  keys.length > 0 && keys.every((name) => present(get(item, name)));

const keyOf = (item: object, keys: readonly string[]): string =>
  keys.map((name) => String(get(item, name) ?? "")).join("\u001f");

const namesOf = (writer: ICRUDRepository<object>, item: object): readonly string[] =>
  writer.ids.length > 0 ? writer.ids : defaultId(Object.keys(item));

const generated = (result: Scalar): Scalar | undefined =>
  result == null || typeof result === "object" ? undefined : result;

const assignIdentity = (item: object, keys: readonly string[], response: Response): void => {
  const scalar = generated(response.result as Scalar);
  if (scalar === undefined) return;
  apply(item, identityCreate(keys).map(scalar));
};

const payload = (parent: object, into: string): object[] => {
  const value = get(parent, into);
  return value === undefined || value === null || typeof value === "string"
    ? []
    : Array.isArray(value)
      ? value.filter((item) => item !== null && typeof item === "object")
      : [];
};

const stamp = (related: object, by: string, key: Scalar | object | undefined): void => {
  if (key !== undefined && key !== null && typeof key !== "object") set(related, by, key);
};

const load = async (
  writer: ICRUDRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<object[]> => {
  const key = get(parent, enrichment.from);
  if (key === undefined || key === null || typeof key === "object") return [];
  const items: object[] = [];
  for await (const item of writer.read(new WhereClause(new Operator.Equals(enrichment.by, key))))
    items.push(item);
  return items;
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
  implements ICRUDRepository<T>, IEnriched<T>
{
  readonly tree: IRepositoryTree;
  readonly ids: readonly string[];
  private readonly delegate?: ICRUDRepository<T>;
  private readonly strategy: IReadonlyEnrichmentStrategy<T>;
  private readonly createMapper?: IMapper<Scalar, T>;
  private readonly idSet: Set<string>;

  constructor(inner: ICRUDRepository<T>, strategy?: IReadonlyEnrichmentStrategy<T>);
  constructor(type: Constructable<T>, connection: IConnection);
  constructor(connection: IConnection, options: CRUDRepositoryOptions<T>);
  constructor(built: BuiltRepository<T>);
  constructor(
    typeOrConnectionOrBuilt: Constructable<T> | IConnection | BuiltRepository<T> | ICRUDRepository<T>,
    connectionOrOptions?: IConnection | CRUDRepositoryOptions<T> | IReadonlyEnrichmentStrategy<T>,
  ) {
    if (isDelegate(typeOrConnectionOrBuilt)) {
      super(typeOrConnectionOrBuilt.type);
      this.delegate = typeOrConnectionOrBuilt;
      this.strategy =
        (connectionOrOptions as IReadonlyEnrichmentStrategy<T> | undefined) ??
        new DefaultReadonlyEnrichmentStrategy<T>();
      this.tree = new RepositoryTree(
        new RepositoryTreeNode(typeOrConnectionOrBuilt, "", undefined, undefined, undefined, this.strategy),
      );
      this.ids = typeOrConnectionOrBuilt.ids;
      this.idSet = new Set(this.ids);
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
    this.idSet = new Set(this.ids);
    this.strategy = new DefaultReadonlyEnrichmentStrategy<T>();
    this.tree = new RepositoryTree(
      new RepositoryTreeNode(this, "", undefined, undefined, undefined, this.strategy),
    );
  }

  private get enriched(): boolean {
    return this.tree.parent.items.length > 0;
  }

  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
    nest?: (node: IRepositoryTreeNode) => void,
  ): this {
    const child = this.tree.parent.include(via, into, from, by, select, many);
    nest?.(child);
    return this;
  }

  enrich = async (items: T[]): Promise<T[]> => {
    await this.strategy.enrich(this.tree, items);
    return items;
  };

  async *[tracked](query?: ReadQuery): AsyncIterable<Sourced<T>> {
    const source = this.delegate ? readTracked(this.delegate, query) : super[tracked](query);
    if (!this.enriched) {
      yield* source;
      return;
    }
    const items: T[] = [];
    const sources: (Row | undefined)[] = [];
    for await (const sourced of source) {
      items.push(sourced.item);
      sources.push(sourced.source);
    }
    await this.strategy.enrich(this.tree, items, sources);
    for (let index = 0; index < items.length; index++)
      yield { item: items[index]!, source: sources[index] };
  }

  create = async (item: T): Promise<Response> => {
    const stored = this.enriched ? this.stripEnriched(item) : item;
    const response = this.delegate ? await this.delegate.create(stored) : await this.insert(stored);
    if (!this.enriched) return response;
    const scalar = generated(response.result as Scalar);
    const names = this.ids.length > 0 ? this.ids : defaultId(Object.keys(item as object));
    const mapper = this.ids.length > 0 && this.createMapper ? this.createMapper : identityCreate<T>(names);
    if (scalar !== undefined) apply(item, mapper.map(scalar));
    await this.persist(this.tree.parent, [item], "create");
    return response;
  };

  update = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const stored = this.enriched ? this.stripEnriched(item) : item;
    const response = this.delegate
      ? await this.delegate.update(stored, byWhere)
      : await this.save(stored, byWhere);
    if (this.enriched) await this.persist(this.tree.parent, [item], "update");
    return response;
  };

  delete = async (item: T, byWhere?: IWhere): Promise<Response> => {
    if (this.enriched) await this.persist(this.tree.parent, [item], "delete");
    return this.delegate ? this.delegate.delete(item, byWhere) : this.remove(item, byWhere);
  };

  readSingleById = async (id: Scalar | Cells): Promise<T | undefined> => {
    if (!this.delegate) return this.readSingle(this.byId(id));
    const item = await this.delegate.readSingleById(id);
    if (!item || !this.enriched) return item;
    await this.enrich([item]);
    return item;
  };

  private insert = async (item: T): Promise<Response> => {
    const fields = fieldsOf(this.reverseConvert(item), (col) => !this.idSet.has(col), this.column, this.cell);
    const response = await execute(this.command(), this.statements!.insert(fields), fields);
    const scalar = asScalar(response.result as string | number | boolean | bigint | null | undefined);
    if (scalar !== undefined && this.createMapper) apply(item, this.createMapper.map(scalar));
    return response;
  };

  private save = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.stored(byWhere) : this.stored(this.idWhere(record), true);
    const fields = fieldsOf(record, (col) => !this.idSet.has(col), this.column, this.cell);
    if (fields.length === 0) throw new DatasourceError("Update requires at least one non-Id field");
    return execute(this.command(), this.statements!.update(fields, where), [
      ...fields,
      ...(where.parameters ?? []),
    ]);
  };

  private remove = async (item: T, byWhere?: IWhere): Promise<Response> => {
    const record = this.reverseConvert(item);
    const where = byWhere ? this.stored(byWhere) : this.stored(this.idWhere(record), true);
    return execute(
      this.command(),
      this.statements!.delete(where),
      where.parameters?.length ? where.parameters : fieldsOf(record, () => true, this.column, this.cell),
    );
  };

  private byId = (id: Scalar | Cells): IWhere => {
    if (this.idSet.size === 0) throw new DatasourceError("ReadSingleById requires an Id column");
    if (isCells(id)) return this.idWhere(id);
    if (this.idSet.size !== 1)
      throw new DatasourceError("Composite id requires a row keyed by each Id column");
    return this.idWhere({ [[...this.idSet][0]!]: id });
  };

  private reverseConvert = (item: T): Cells =>
    this.rowConverter!.reverseConvert(this.mapper!.reverseMap(item));

  private idWhere = (record: Cells): IWhere => {
    const parts: IExpression[] = [...this.idSet].map((col) => {
      if (!Object.prototype.hasOwnProperty.call(record, col))
        throw new DatasourceError(`Missing Id value "${col}"`);
      return new Operator.Equals(col, record[col] as Scalar);
    });
    return new WhereClause(
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

  private persist = async (node: IRepositoryTreeNode, parents: object[], mode: Mode): Promise<void> => {
    for (const child of node.items) await this.persistNode(child, parents, mode);
  };

  private persistNode = async (
    child: IRepositoryTreeNode,
    parents: object[],
    mode: Mode,
  ): Promise<void> => {
    const enrichment = child.enrichment;
    if (!enrichment?.many || enrichment.select) return;
    const writer = writerOf(child.value, enrichment.into);
    for (const parent of parents)
      await ({
        create: () => this.createChildren(child, writer, enrichment, parent),
        update: () => this.syncChildren(child, writer, enrichment, parent),
        delete: () => this.deleteChildren(child, writer, enrichment, parent),
      }[mode] ??
        (() => {
          throw new DatasourceError("Unknown persist mode");
        }))();
  };

  private createChildren = async (
    node: IRepositoryTreeNode,
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    for (const related of payload(parent, enrichment.into)) {
      stamp(related, enrichment.by, get(parent, enrichment.from));
      assignIdentity(related, namesOf(writer, related), await writer.create(this.stripNode(node, related)));
      await this.persist(node, [related], "create");
    }
  };

  private syncChildren = async (
    node: IRepositoryTreeNode,
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    const existing = await load(writer, enrichment, parent);
    const rows = payload(parent, enrichment.into);
    for (const related of rows) stamp(related, enrichment.by, get(parent, enrichment.from));
    const keys = namesOf(writer, rows[0] ?? existing[0] ?? {});
    const keep = new Set(rows.filter((row) => hasKey(row, keys)).map((row) => keyOf(row, keys)));
    for (const row of existing.filter((row) => !keep.has(keyOf(row, keys)))) {
      await this.persist(node, [row], "delete");
      await writer.delete(row);
    }
    for (const related of rows) {
      hasKey(related, keys) && existing.some((row) => keyOf(row, keys) === keyOf(related, keys))
        ? await writer.update(this.stripNode(node, related))
        : assignIdentity(related, keys, await writer.create(this.stripNode(node, related)));
      await this.persist(node, [related], "update");
    }
  };

  private deleteChildren = async (
    node: IRepositoryTreeNode,
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    const existing = await load(writer, enrichment, parent);
    await this.persist(node, existing, "delete");
    for (const row of existing) await writer.delete(row);
  };
}
