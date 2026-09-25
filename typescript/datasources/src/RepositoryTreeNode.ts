import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyEnrichmentBinder } from "./IReadonlyEnrichmentBinder.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { ITrackedApply } from "./ITrackedApply.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { Row } from "./Row.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

export class RepositoryTreeNode implements IRepositoryTreeNode {
  readonly items: RepositoryTreeNode[] = [];
  private readonly apply: (parents: object[], sources?: (Row | undefined)[]) => Promise<void>;
  private readonly binder: IReadonlyEnrichmentBinder;
  persist?: ICRUDRepository<object>;
  link?: ICRUDRepository<object>;

  constructor(
    readonly value: object,
    readonly propertyName: string,
    readonly parent?: IRepositoryTreeNode,
    apply: (parents: object[], sources?: (Row | undefined)[]) => Promise<void> = async () => {},
    readonly enrichment?: Enrichment,
    binder?: IReadonlyEnrichmentBinder,
  ) {
    this.binder = binder ?? new DefaultReadonlyEnrichmentStrategy<object>();
    this.apply = apply;
  }

  include<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    select?: string,
    many?: boolean,
  ): IRepositoryTreeNode {
    return this.includeWith(via, into, from, by, select, many, this.binder);
  }

  /** @internal used by addEnriched to store strategy on the child */
  includeWith<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    select: string | undefined,
    many: boolean | undefined,
    strategy: IReadonlyEnrichmentBinder,
  ): RepositoryTreeNode {
    const enrichment: Enrichment = { into, from, by, select, many };
    const tracked = strategy as ITrackedApply;
    const child = new RepositoryTreeNode(
      via,
      into,
      this,
      (parents, sources) => tracked.applyTracked(via, enrichment, parents, sources),
      enrichment,
      strategy,
    );
    if (isWriter(via)) child.persist = via as ICRUDRepository<object>;
    this.items.push(child);
    return child;
  }

  includeLinked<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    link: ICRUDRepository<TLink>,
    parent: string,
    related: string,
  ): IRepositoryTreeNode {
    return this.includeLinkedWith(via, into, from, by, link, parent, related, this.binder);
  }

  /** @internal used by addLinked to store strategy on the child */
  includeLinkedWith<TRelated extends object, TLink extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    link: ICRUDRepository<TLink>,
    parent: string,
    related: string,
    strategy: IReadonlyEnrichmentBinder,
  ): RepositoryTreeNode {
    const writer = link as ICRUDRepository<object>;
    const enrichment: Enrichment = {
      into,
      from,
      by,
      many: true,
      association: { parent, related },
    };
    const child = new RepositoryTreeNode(
      via,
      into,
      this,
      (parents, sources) => applyLinked(via, writer, enrichment, parents, sources),
      enrichment,
      strategy,
    );
    child.link = writer;
    this.items.push(child);
    return child;
  }

  enrichFrom = async (parents: object[], sources?: (Row | undefined)[]): Promise<void> => {
    await this.apply(parents, sources);
    const next = related(parents, this.propertyName);
    const nested = this.propertyName ? undefined : sources;
    for (const child of this.items) await child.enrichFrom(next, nested);
  };
}

const isWriter = (via: object): via is ICRUDRepository<object> =>
  "create" in via && typeof via.create === "function";

const applyLinked = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  link: ICRUDRepository<object>,
  enrichment: Enrichment,
  parents: object[],
  sources?: (Row | undefined)[],
): Promise<void> => {
  const association = enrichment.association!;
  for (const [index, parent] of parents.entries()) {
    const key = sourceKey(parent, enrichment.from, sources, index);
    if (key === undefined || key === null) continue;
    const related: TRelated[] = [];
    for await (const row of link.read(
      new Where(new Operator.Equals(association.parent, scalar(key))),
    )) {
      const id = props(row)[association.related];
      if (id === undefined || id === null || typeof id === "object") continue;
      const entity = await via.readSingle(new Where(new Operator.Equals(enrichment.by, id)));
      if (entity) related.push(entity);
    }
    Object.assign(parent, { [enrichment.into]: related });
  }
};

export const sourceKey = (
  parent: object,
  name: string,
  sources: (Row | undefined)[] | undefined,
  index: number,
): Scalar | object | undefined => {
  const cell = sources?.[index]?.[name];
  if (cell !== undefined && cell !== null) return cell;
  return props(parent)[name];
};

export const props = (item: object): Props => item as Props;

export const keyOf = (key: Scalar | object): string => String(key);

export const scalar = (value: Scalar | object): Scalar =>
  value === null ||
  typeof value === "string" ||
  typeof value === "number" ||
  typeof value === "boolean" ||
  typeof value === "bigint"
    ? value
    : (() => {
        throw new DatasourceError("Enrichment key must be a scalar");
      })();

const related = (parents: object[], propertyName: string): object[] => {
  if (!propertyName) return parents;
  const next: object[] = [];
  for (const parent of parents) {
    const value = props(parent)[propertyName];
    if (value === undefined || value === null || typeof value === "string") continue;
    if (Array.isArray(value)) {
      for (const item of value)
        if (item !== null && typeof item === "object") next.push(item);
      continue;
    }
    if (typeof value === "object") next.push(value);
  }
  return next;
};
