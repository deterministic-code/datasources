import { WhereBuilder } from "./builders/WhereBuilder.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { Scalar } from "./builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

export class RepositoryTreeNode implements IRepositoryTreeNode {
  readonly items: RepositoryTreeNode[] = [];

  constructor(
    readonly value: object,
    readonly propertyName: string,
    readonly parent?: IRepositoryTreeNode,
    private readonly apply: (parents: object[]) => Promise<void> = async () => {},
  ) {}

  include<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    into: string,
    from: string,
    by: string,
    select?: string,
    many?: boolean,
  ): this {
    const enrichment: Enrichment = { into, from, by, select, many };
    this.items.push(
      new RepositoryTreeNode(via, into, this, (parents) =>
        apply(via, enrichment, parents),
      ),
    );
    return this;
  }

  enrichFrom = async (parents: object[]): Promise<void> => {
    await this.apply(parents);
    for (const child of this.items) await child.enrichFrom(parents);
  };
}

const apply = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> =>
  enrichment.many
    ? applyMany(via, enrichment, parents)
    : applyLookup(via, enrichment, parents);

const applyLookup = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> => {
  const cache = new Map<string, TRelated | undefined>();
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
    if (key === undefined || key === null) continue;
    const cacheKey = String(key);
    if (!cache.has(cacheKey))
      cache.set(
        cacheKey,
        await via.getSingle(WhereBuilder.equals(enrichment.by, scalar(key))),
      );
    const related = cache.get(cacheKey);
    if (!related) continue;
    const value = enrichment.select
      ? props(related)[enrichment.select]
      : related;
    Object.assign(parent, { [enrichment.into]: value });
  }
};

const applyMany = async <TRelated extends object>(
  via: IReadonlyRepository<TRelated>,
  enrichment: Enrichment,
  parents: object[],
): Promise<void> => {
  for (const parent of parents) {
    const key = props(parent)[enrichment.from];
    if (key === undefined || key === null) continue;
    const related: TRelated[] = [];
    for await (const item of via.get(
      WhereBuilder.equals(enrichment.by, scalar(key)),
    ))
      related.push(item);
    Object.assign(parent, { [enrichment.into]: related });
  }
};

const props = (item: object): Props => item as Props;

const scalar = (value: Scalar | object): Scalar =>
  value === null ||
  typeof value === "string" ||
  typeof value === "number" ||
  typeof value === "boolean" ||
  typeof value === "bigint"
    ? value
    : (() => {
        throw new DatasourceError("Enrichment key must be a scalar");
      })();
