import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { IReadonlyEnrichmentBinder } from "./IReadonlyEnrichmentBinder.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { IRepositoryTreeNode } from "./IRepositoryTreeNode.ts";
import type { Scalar } from "./builders/Scalar.ts";

type Props = Record<string, Scalar | object>;

export class RepositoryTreeNode implements IRepositoryTreeNode {
  readonly items: RepositoryTreeNode[] = [];
  private readonly apply: (parents: object[]) => Promise<void>;
  private readonly binder: IReadonlyEnrichmentBinder;

  constructor(
    readonly value: object,
    readonly propertyName: string,
    readonly parent?: IRepositoryTreeNode,
    apply: (parents: object[]) => Promise<void> = async () => {},
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
    const enrichment: Enrichment = { into, from, by, select, many };
    const child = new RepositoryTreeNode(
      via,
      into,
      this,
      (parents) => this.binder.apply(via, enrichment, parents),
      enrichment,
      this.binder,
    );
    this.items.push(child);
    return child;
  }

  enrichFrom = async (parents: object[]): Promise<void> => {
    await this.apply(parents);
    const next = related(parents, this.propertyName);
    for (const child of this.items) await child.enrichFrom(next);
  };
}

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
    if (value === undefined || value === null || typeof value === "string")
      continue;
    if (Array.isArray(value)) {
      for (const item of value)
        if (item !== null && typeof item === "object") next.push(item);
      continue;
    }
    if (typeof value === "object") next.push(value);
  }
  return next;
};
