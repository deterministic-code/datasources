import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

const linkKey = (value: Scalar | object | undefined): Scalar =>
  value === undefined || value === null || typeof value === "object" ? null : value;

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

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
    const key = get(parent, enrichment.sourceProperty);
    if (key === undefined || key === null) return [];
    const items: TChild[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(enrichment.relatedKey, key))))
      items.push(item);
    return items;
  };

  static childrenWhere = <TParent, TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: TParent,
  ): Where | undefined => {
    if (writer.ids.length > 0) return undefined;
    const key = get(parent, enrichment.sourceProperty);
    return key === undefined || key === null
      ? undefined
      : new Where(new Operator.Equals(enrichment.relatedKey, key));
  };

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
