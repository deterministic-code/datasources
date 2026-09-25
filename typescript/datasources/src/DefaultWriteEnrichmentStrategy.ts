import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import { RepositoryTreeNode } from "./RepositoryTreeNode.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { defaultId } from "./createRepositoryOptions.ts";
import { Operator, Where } from "./statements/index.ts";
import type { WriteMode } from "./WriteMode.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const get = (item: object, name: string): Scalar | object | undefined => props(item)[name];

const set = (item: object, name: string, value: Scalar): void => {
  Object.assign(item, { [name]: value });
};

const present = (value: Scalar | object | undefined): value is Scalar =>
  value !== undefined && value !== null && typeof value !== "object" && value !== 0 && value !== 0n && value !== "";

const unloaded = (parent: object, into: string): boolean => {
  const value = get(parent, into);
  return value === undefined || value === null;
};

const hasKey = (item: object, keys: readonly string[]): boolean =>
  keys.length > 0 && keys.every((name) => present(get(item, name)));

const keyOf = (item: object, keys: readonly string[]): string =>
  keys.map((name) => String(get(item, name) ?? "")).join("\u001f");

const namesOf = (writer: ICRUDRepository<object>, item: object): readonly string[] =>
  writer.ids.length > 0 ? writer.ids : defaultId(Object.keys(item));

const assignIdentity = (item: object, keys: readonly string[], created: object): void => {
  if (item === created) return;
  const source = props(created);
  for (const key of keys) {
    const value = source[key];
    if (value !== undefined && value !== null && typeof value !== "object") props(item)[key] = value;
  }
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

const linkWhere = (
  association: NonNullable<Enrichment["association"]>,
  parentKey: Scalar,
  relatedKey: Scalar,
): Where =>
  new Where(
    new Operator.And(
      new Operator.Equals(association.parent, parentKey),
      new Operator.Equals(association.related, relatedKey),
    ),
  );

const relatedIds = (parent: object, enrichment: Enrichment): Scalar[] => {
  const ids: Scalar[] = [];
  const seen = new Set<string>();
  for (const related of payload(parent, enrichment.into)) {
    const id = get(related, enrichment.by);
    if (!present(id)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.by}"`);
    const key = String(id);
    if (seen.has(key)) continue;
    seen.add(key);
    ids.push(id);
  }
  return ids;
};

const requiredKey = (parent: object, enrichment: Enrichment): Scalar => {
  const key = get(parent, enrichment.from);
  if (!present(key)) throw new DatasourceError(`${enrichment.into} requires "${enrichment.from}"`);
  return key;
};

const createLinks = async (
  writer: ICRUDRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<void> => {
  const association = enrichment.association!;
  const parentKey = requiredKey(parent, enrichment);
  for (const id of relatedIds(parent, enrichment))
    await writer.create({ [association.parent]: parentKey, [association.related]: id });
};

const syncLinks = async (
  writer: ICRUDRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<void> => {
  const association = enrichment.association!;
  const parentKey = requiredKey(parent, enrichment);
  const existing = await DeleteEnrichmentStrategy.loadLinks(writer, association.parent, parentKey);
  const wanted = relatedIds(parent, enrichment);
  const keep = new Set(wanted.map((id) => String(id)));
  for (const row of existing) {
    const id = get(row, association.related);
    if (present(id) && keep.has(String(id))) continue;
    if (!present(id)) continue;
    await writer.delete(row, linkWhere(association, parentKey, id));
  }
  for (const id of wanted) {
    if (existing.some((row) => String(get(row, association.related) ?? "") === String(id))) continue;
    await writer.create({ [association.parent]: parentKey, [association.related]: id });
  }
};

const isWriter = (via: object): via is ICRUDRepository<object> =>
  "create" in via && typeof via.create === "function";

const writerOf = (via: object, into: string): ICRUDRepository<object> =>
  isWriter(via)
    ? via
    : (() => {
        throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
      })();

const linkWriter = (child: IRepositoryTreeNode, into: string): ICRUDRepository<object> => {
  const link = (child as RepositoryTreeNode).link;
  if (!link) throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
  return link;
};

const stripNode = (node: IRepositoryTreeNode, item: object): object => {
  const copy = { ...item } as Props;
  for (const child of node.items)
    if (child.enrichment) delete copy[child.enrichment.into];
  return copy;
};

const dispatch = (mode: string, runs: Record<WriteMode, () => Promise<void>>): Promise<void> =>
  (runs[mode as WriteMode] ??
    (() => {
      throw new DatasourceError("Unknown persist mode");
    }))();

export class DefaultWriteEnrichmentStrategy<T> implements IWriteEnrichmentStrategy<T> {
  protected skipUnloaded = false;

  constructor(private readonly deleteStrategy = new CascadeDeleteEnrichmentStrategy<T>()) {}

  persist = async (tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void> => {
    await this.walk(tree.parent, items as object[], mode);
  };

  private walk = async (node: IRepositoryTreeNode, parents: object[], mode: WriteMode): Promise<void> => {
    for (const child of node.items) await this.persistNode(child, parents, mode);
  };

  private persistNode = async (
    child: IRepositoryTreeNode,
    parents: object[],
    mode: WriteMode,
  ): Promise<void> => {
    const enrichment = child.enrichment;
    if (enrichment?.association) {
      const writer = linkWriter(child, enrichment.into);
      for (const parent of parents) {
        if (this.skip(parent, enrichment.into, mode)) continue;
        await dispatch(mode, {
          create: () => createLinks(writer, enrichment, parent),
          update: () => syncLinks(writer, enrichment, parent),
          delete: () => this.deleteStrategy.deleteLinked(child, enrichment, parent),
        });
      }
      return;
    }
    if (!enrichment?.many || enrichment.select) return;
    const writer = writerOf(child.value, enrichment.into);
    for (const parent of parents) {
      if (this.skip(parent, enrichment.into, mode)) continue;
      await dispatch(mode, {
        create: () => this.createChildren(child, writer, enrichment, parent),
        update: () => this.syncChildren(child, writer, enrichment, parent),
        delete: () =>
          this.deleteStrategy.deleteOwned(child, enrichment, parent, (rows) => this.walk(child, rows, "delete")),
      });
    }
  };

  private skip = (parent: object, into: string, mode: string): boolean =>
    this.skipUnloaded && mode !== "delete" && unloaded(parent, into);

  private createChildren = async (
    node: IRepositoryTreeNode,
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    for (const related of payload(parent, enrichment.into)) {
      stamp(related, enrichment.by, get(parent, enrichment.from));
      assignIdentity(related, namesOf(writer, related), await writer.create(stripNode(node, related)));
      await this.walk(node, [related], "create");
    }
  };

  private syncChildren = async (
    node: IRepositoryTreeNode,
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    const existing = await DeleteEnrichmentStrategy.load(writer, enrichment, parent);
    const rows = payload(parent, enrichment.into);
    for (const related of rows) stamp(related, enrichment.by, get(parent, enrichment.from));
    const keys = namesOf(writer, rows[0] ?? existing[0] ?? {});
    const keep = new Set(rows.filter((row) => hasKey(row, keys)).map((row) => keyOf(row, keys)));
    for (const row of existing.filter((row) => !keep.has(keyOf(row, keys)))) {
      await this.walk(node, [row], "delete");
      await writer.delete(row);
    }
    for (const related of rows) {
      hasKey(related, keys) && existing.some((row) => keyOf(row, keys) === keyOf(related, keys))
        ? await writer.update(stripNode(node, related))
        : assignIdentity(related, keys, await writer.create(stripNode(node, related)));
      await this.walk(node, [related], "update");
    }
  };
}
