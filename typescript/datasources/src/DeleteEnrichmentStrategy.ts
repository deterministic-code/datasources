import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

export abstract class DeleteEnrichmentStrategy<T> {
  abstract deleteOwned<TChild extends object>(
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
    if (key === undefined || key === null || typeof key === "object") return [];
    const items: TChild[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(enrichment.relatedKey, key))))
      items.push(item);
    return items;
  };

  static ownedWhere = <TParent, TChild extends object>(
    writer: ICRUDRepository<TChild>,
    enrichment: Enrichment,
    parent: TParent,
  ): Where | undefined => {
    if (writer.ids.length > 0) return undefined;
    const key = get(parent, enrichment.sourceProperty);
    return key === undefined || key === null || typeof key === "object"
      ? undefined
      : new Where(new Operator.Equals(enrichment.relatedKey, key));
  };

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
