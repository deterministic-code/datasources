import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const get = <TItem>(item: TItem, name: string): Scalar | object | undefined =>
  (item as never as Props)[name];

export abstract class DeleteEnrichmentStrategy<T> {
  abstract deleteOwned(
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: T,
    deleteNested: (rows: object[]) => Promise<void>,
  ): Promise<void>;

  abstract deleteLinked(writer: ICRUDRepository<object>, enrichment: Enrichment, parent: T): Promise<void>;

  static load = async <TParent>(
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: TParent,
  ): Promise<object[]> => {
    const key = get(parent, enrichment.sourceProperty);
    if (key === undefined || key === null || typeof key === "object") return [];
    const items: object[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(enrichment.relatedKey, key))))
      items.push(item);
    return items;
  };

  static loadLinks = async (
    writer: ICRUDRepository<object>,
    column: string,
    key: Scalar,
  ): Promise<object[]> => {
    const items: object[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(column, key))))
      items.push(item);
    return items;
  };
};
