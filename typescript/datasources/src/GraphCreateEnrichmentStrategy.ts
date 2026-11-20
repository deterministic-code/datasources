import { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Enrichment } from "./Enrichment.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { Scalar } from "./builders/Scalar.ts";
import { nodeKey } from "./TreeCreateEnrichmentStrategy.ts";

export class GraphCreateEnrichmentStrategy<T extends object, TLink extends object = object> extends CreateEnrichmentStrategy<T> {
  private readonly inserted = new Map<string, Scalar>();
  private depth = 0;

  constructor(private readonly link: ICRUDRepository<TLink>) {
    super();
  }

  createLinked = async <TChild extends object>(
    _writer: ICRUDRepository<TChild>,
    _enrichment: Enrichment,
    _parent: T,
  ): Promise<void> => {};

  createChildren = async <TChild extends object>(writer: ICRUDRepository<TChild>, enrichment: Enrichment, parent: T): Promise<void> => {
    const association = enrichment.association ?? (() => {
      throw new DatasourceError("Missing association");
    })();
    this.depth += 1;
    try {
      const parentId = this.requiredValue(parent, enrichment.sourceProperty, enrichment.destinationProperty);
      this.inserted.set(nodeKey(parent), parentId);
      const children = this.payloadOf<TChild, T>(parent, enrichment.destinationProperty, enrichment, writer);
      for (const child of children) {
        const key = nodeKey(child);
        let relatedId = this.inserted.get(key);
        if (relatedId === undefined) {
          const created = await writer.create(child);
          relatedId = this.requiredValue(created, enrichment.sourceProperty, enrichment.destinationProperty);
          this.inserted.set(key, relatedId);
          this.stampCell(child, enrichment.sourceProperty, relatedId);
          await this.createChildren(writer, enrichment, child as never as T);
        }
        await this.link.create({ [association.parent]: parentId, [association.related]: relatedId } as TLink);
      }
    } finally {
      this.depth -= 1;
      if (this.depth === 0) this.inserted.clear();
    }
  };
}
