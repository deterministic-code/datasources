import type { Enrichment } from "./Enrichment.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";

export class GraphReadonlyEnrichmentStrategy<T extends object, TLink extends object = object>
  implements IReadonlyEnrichmentStrategy<T>
{
  private readonly keys = new DefaultReadonlyEnrichmentStrategy<T>();

  constructor(private readonly link: IReadonlyRepository<TLink>) {}

  apply<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    return this.fill(via, enrichment, parents, sources);
  }

  applyLinked<TRelated extends object, TJunction extends object>(
    via: IReadonlyRepository<TRelated>,
    _link: IReadonlyRepository<TJunction>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    return this.apply(via, enrichment, parents, sources);
  }

  private async fill<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const association = enrichment.association ?? (() => {
      throw new DatasourceError("Missing association");
    })();
    const nodes = new Map<string, TRelated>();
    for await (const item of via.read()) {
      const id = this.keys.matchKey(item, enrichment);
      if (id !== undefined) nodes.set(id, item);
    }
    const outgoing = new Map<string, string[]>();
    for await (const edge of this.link.read()) {
      const row = edge as Record<string, string | number | boolean | bigint | null>;
      const from = row[association.parent];
      const to = row[association.related];
      if (from === undefined || from === null || to === undefined || to === null) continue;
      const fromKey = this.keys.keyOf(from as never);
      const toKey = this.keys.keyOf(to as never);
      const list = outgoing.get(fromKey) ?? [];
      list.push(toKey);
      outgoing.set(fromKey, list);
    }
    const filled = new Set<string>();
    for (const [offset, parent] of parents.entries()) {
      const id = this.keys.matchKey(parent, enrichment, sources, offset);
      if (id === undefined) {
        Object.assign(parent, { [enrichment.destinationProperty]: [] });
        continue;
      }
      filled.add(id);
      Object.assign(parent, {
        [enrichment.destinationProperty]: this.walk(id, enrichment, nodes, outgoing, new Set([id]), filled),
      });
    }
  }

  private walk<TRelated extends object>(
    from: string,
    enrichment: Enrichment,
    nodes: Map<string, TRelated>,
    outgoing: Map<string, string[]>,
    path: Set<string>,
    filled: Set<string>,
  ): TRelated[] {
    const edges: TRelated[] = [];
    for (const to of outgoing.get(from) ?? []) {
      if (path.has(to)) continue;
      const node = nodes.get(to);
      if (node === undefined) continue;
      const first = !filled.has(to);
      if (first) filled.add(to);
      const copy = { ...node, [enrichment.destinationProperty]: [] } as TRelated;
      if (first) {
        path.add(to);
        Object.assign(copy, {
          [enrichment.destinationProperty]: this.walk(to, enrichment, nodes, outgoing, path, filled),
        });
        path.delete(to);
      }
      edges.push(copy);
    }
    return edges;
  }
}
