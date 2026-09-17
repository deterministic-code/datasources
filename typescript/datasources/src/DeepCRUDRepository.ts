import { WhereBuilder } from "./builders/WhereBuilder.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { EnrichedReadonlyRepository } from "./EnrichedReadonlyRepository.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IRepository } from "./IRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { IResponse } from "./IResponse.ts";
import type { IWhere } from "./IWhere.ts";

type Props = Record<string, Scalar | object>;
type Mode = "create" | "update" | "delete";

const props = (item: object): Props => item as Props;

const isRepository = (via: object): via is IRepository<object> =>
  "add" in via && typeof via.add === "function";

const writerOf = (via: object, into: string): IRepository<object> =>
  isRepository(via)
    ? via
    : (() => {
        throw new DatasourceError(
          `Deep write requires a CRUD repository for "${into}"`,
        );
      })();

const get = (item: object, name: string): Scalar | object | undefined =>
  props(item)[name];

const set = (item: object, name: string, value: Scalar): void => {
  Object.assign(item, { [name]: value });
};

const idOf = (item: object): Scalar | undefined => {
  const value = get(item, "id") ?? get(item, "Id");
  return value === undefined || typeof value === "object" ? undefined : value;
};

const hasId = (id: Scalar | undefined): id is Scalar =>
  id !== undefined && id !== null && id !== 0 && id !== 0n && id !== "";

const idKey = (id: Scalar | undefined): string => String(id ?? "");

const payload = (parent: object, into: string): object[] => {
  const value = get(parent, into);
  return value === undefined || value === null || typeof value === "string"
    ? []
    : Array.isArray(value)
      ? value.filter((item) => item !== null && typeof item === "object")
      : [];
};

const stamp = (related: object, by: string, key: Scalar | object | undefined): void => {
  if (key !== undefined && key !== null && typeof key !== "object")
    set(related, by, key);
};

const applyIdentity = (item: object, response: IResponse): void => {
  if (response.result === undefined || response.result === null) return;
  const name = "id" in item ? "id" : "Id" in item ? "Id" : undefined;
  if (name === undefined) return;
  const result = response.result;
  if (
    typeof result === "string" ||
    typeof result === "number" ||
    typeof result === "boolean" ||
    typeof result === "bigint"
  )
    set(item, name, result);
};

const load = async (
  writer: IRepository<object>,
  enrichment: Enrichment,
  parent: object,
): Promise<object[]> => {
  const key = get(parent, enrichment.from);
  if (key === undefined || key === null || typeof key === "object") return [];
  const items: object[] = [];
  for await (const item of writer.get(WhereBuilder.equals(enrichment.by, key)))
    items.push(item);
  return items;
};

export class DeepCRUDRepository<T extends object>
  extends EnrichedReadonlyRepository<T>
  implements IRepository<T>
{
  constructor(private readonly crud: IRepository<T>) {
    super(crud);
  }

  add = async (item: T): Promise<IResponse> => {
    const response = await this.crud.add(this.stripEnriched(item));
    applyIdentity(item, response);
    await this.persist(this.tree.parent, [item], "create");
    return response;
  };

  update = async (item: T, byWhere?: IWhere): Promise<IResponse> => {
    const response = await this.crud.update(this.stripEnriched(item), byWhere);
    await this.persist(this.tree.parent, [item], "update");
    return response;
  };

  private stripEnriched = (item: T): T => this.stripNode(this.tree.parent, item) as T;

  private stripNode = (node: IRepositoryTreeNode, item: object): object => {
    const copy = { ...item } as Props;
    for (const child of node.items)
      if (child.enrichment) delete copy[child.enrichment.into];
    return copy;
  };

  delete = async (item: T, byWhere?: IWhere): Promise<IResponse> => {
    await this.persist(this.tree.parent, [item], "delete");
    return this.crud.delete(item, byWhere);
  };

  private persist = async (
    node: IRepositoryTreeNode,
    parents: object[],
    mode: Mode,
  ): Promise<void> => {
    for (const child of node.items)
      await this.persistNode(child, parents, mode);
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
    writer: IRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    for (const related of payload(parent, enrichment.into)) {
      stamp(related, enrichment.by, get(parent, enrichment.from));
      applyIdentity(related, await writer.add(this.stripNode(node, related)));
      await this.persist(node, [related], "create");
    }
  };

  private syncChildren = async (
    node: IRepositoryTreeNode,
    writer: IRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    const existing = await load(writer, enrichment, parent);
    const rows = payload(parent, enrichment.into);
    for (const related of rows)
      stamp(related, enrichment.by, get(parent, enrichment.from));
    const keep = new Set(
      rows.map(idOf).filter(hasId).map(idKey),
    );
    for (const row of existing.filter((row) => !keep.has(idKey(idOf(row))))) {
      await this.persist(node, [row], "delete");
      await writer.delete(row);
    }
    for (const related of rows) {
      const id = idOf(related);
      hasId(id) && existing.some((row) => idKey(idOf(row)) === idKey(id))
        ? await writer.update(this.stripNode(node, related))
        : applyIdentity(related, await writer.add(this.stripNode(node, related)));
      await this.persist(node, [related], "update");
    }
  };

  private deleteChildren = async (
    node: IRepositoryTreeNode,
    writer: IRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<void> => {
    const existing = await load(writer, enrichment, parent);
    await this.persist(node, existing, "delete");
    for (const row of existing) await writer.delete(row);
  };
}
