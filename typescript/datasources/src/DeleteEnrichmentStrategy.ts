import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { Operator, Where } from "./statements/index.ts";

type Props = Record<string, Scalar | object>;

const props = (item: object): Props => item as Props;

const get = (item: object, name: string): Scalar | object | undefined => props(item)[name];

export abstract class DeleteEnrichmentStrategy<T> {
  declare private readonly subject: T; // keeps T paired with the write strategy

  abstract deleteOwned(
    entry: StoredEnrichment,
    enrichment: Enrichment,
    parent: object,
    deleteNested: (rows: object[]) => Promise<void>,
  ): Promise<void>;

  abstract deleteLinked(entry: StoredEnrichment, enrichment: Enrichment, parent: object): Promise<void>;

  static load = async (
    writer: ICRUDRepository<object>,
    enrichment: Enrichment,
    parent: object,
  ): Promise<object[]> => {
    const key = get(parent, enrichment.from);
    if (key === undefined || key === null || typeof key === "object") return [];
    const items: object[] = [];
    for await (const item of writer.read(new Where(new Operator.Equals(enrichment.by, key))))
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
}
