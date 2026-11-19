import { enrichmentMatches, type Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

const linkKey = (value: Scalar | object | undefined): Scalar =>
  value === undefined || value === null || typeof value === "object" ? null : value;

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

const matchWhere = <TParent>(enrichment: Enrichment, parent: TParent): Where | undefined => {
  const parts: Operator[] = [];
  for (const match of enrichmentMatches(enrichment)) {
    const key = get(parent, match.sourceProperty);
    if (key === undefined || key === null) return undefined;
    parts.push(new Operator.Equals(match.relatedKey, key as Scalar));
  }
  const [first, ...rest] = parts;
  return first === undefined ? undefined : new Where(rest.reduce((left, right) => new Operator.And(left, right), first));
};

export abstract class DeleteEnrichmentStrategy<T> {
  abstract deleteChildren<TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: T,
    deleteNested: (rows: TChild[]) => Promise<void>,
  ): Promise<void>;

  abstract deleteLinked<TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void>;

  static load = async <TParent, TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: TParent,
  ): Promise<TChild[]> => {
    const where = matchWhere(enrichment, parent);
    if (where === undefined) return [];
    const items: TChild[] = [];
    for await (const item of writer.read(where))
      items.push(item);
    return items;
  };

  static childrenWhere = <TParent, TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: TParent,
  ): Where | undefined => (writer.ids.length > 0 ? undefined : matchWhere(enrichment, parent));

  static linkKey = linkKey;

  static linkWhere = (
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

  static loadLinks = async <TChild extends object>(
    writer: ICRUDRepository<TChild>,
    column: string,
    key: Scalar,
  ): Promise<TChild[]> => {
    const items: TChild[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(column, key))))
      items.push(item);
    return items;
  };
};
