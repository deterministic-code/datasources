import type { Enrichment } from "./Enrichment.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultReadonlyEnrichmentStrategy } from "./DefaultReadonlyEnrichmentStrategy.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";
import type { Row } from "./Row.ts";

export class TreeReadonlyEnrichmentStrategy<T extends object>
  implements IReadonlyEnrichmentStrategy<T>
{
  private readonly keys = new DefaultReadonlyEnrichmentStrategy<T>();

  apply<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    return this.fill(via, enrichment, parents, sources);
  }

  applyLinked<TRelated extends object, TLink extends object>(
    _via: IReadonlyRepository<TRelated>,
    _link: IReadonlyRepository<TLink>,
    _enrichment: Enrichment,
    _parents: T[],
    _sources?: (Row | undefined)[],
  ): Promise<void> {
    return Promise.reject(new DatasourceError("Tree enrichment does not use a junction"));
  }

  private async fill<TRelated extends object>(
    via: IReadonlyRepository<TRelated>,
    enrichment: Enrichment,
    parents: T[],
    sources?: (Row | undefined)[],
  ): Promise<void> {
    const rows: TRelated[] = [];
    for await (const item of via.read()) rows.push(item);
    const index = new Map<string, TRelated[]>();
    for (const item of rows) {
      const parentKey = this.keys.rowMatchKey(item, enrichment);
      if (parentKey === undefined) continue;
      const list = index.get(parentKey) ?? [];
      list.push(item);
      index.set(parentKey, list);
    }
    for (const [offset, parent] of parents.entries())
      this.attach(parent, enrichment, index, new Set(), sources, offset);
  }

  private attach<TRelated extends object>(
    parent: T,
    enrichment: Enrichment,
    index: Map<string, TRelated[]>,
    path: Set<string>,
    sources: (Row | undefined)[] | undefined,
    offset: number,
  ): void {
    const id = this.keys.matchKey(parent, enrichment, sources, offset);
    if (id === undefined) {
      Object.assign(parent, { [enrichment.destinationProperty]: [] });
      return;
    }
    path.add(id);
    const children: TRelated[] = [];
    for (const child of index.get(id) ?? []) {
      const childId = this.keys.matchKey(child, enrichment);
      if (childId === undefined || path.has(childId)) continue;
      const copy = this.copy(child, enrichment.destinationProperty);
      this.nest(copy, enrichment, index, path);
      children.push(copy);
    }
    path.delete(id);
    Object.assign(parent, { [enrichment.destinationProperty]: children });
  }

  private nest<TRelated extends object>(
    node: TRelated,
    enrichment: Enrichment,
    index: Map<string, TRelated[]>,
    path: Set<string>,
  ): void {
    const id = this.keys.matchKey(node, enrichment);
    if (id === undefined) {
      Object.assign(node, { [enrichment.destinationProperty]: [] });
      return;
    }
    path.add(id);
    const children: TRelated[] = [];
    for (const child of index.get(id) ?? []) {
      const childId = this.keys.matchKey(child, enrichment);
      if (childId === undefined || path.has(childId)) continue;
      const copy = this.copy(child, enrichment.destinationProperty);
      this.nest(copy, enrichment, index, path);
      children.push(copy);
    }
    path.delete(id);
    Object.assign(node, { [enrichment.destinationProperty]: children });
  }

  private copy<TRelated extends object>(item: TRelated, dest: string): TRelated {
    return { ...item, [dest]: [] };
  }
}
