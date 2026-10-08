import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import type { CreateEnrichmentStrategy } from "./CreateEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultCreateEnrichmentStrategy } from "./DefaultCreateEnrichmentStrategy.ts";
import { EnrichmentPropertyType } from "./EnrichmentPropertyType.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { enrichmentsOf } from "./ReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { DeleteEnrichmentStrategy } from "./DeleteEnrichmentStrategy.ts";
import type { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";
import type { WriteMode } from "./WriteMode.ts";

const unloaded = (parent: object, into: string): boolean => {
  const value = (parent as Record<string, Scalar | object>)[into];
  return value === undefined || value === null;
};

const childWriter = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.persist) return entry.persist;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

const linkWriter = (entry: StoredEnrichment, into: string): ICRUDRepository<object> => {
  if (entry.link) return entry.link;
  throw new DatasourceError(`Deep write requires a CRUD repository for "${into}"`);
};

const dispatch = (mode: string, runs: Record<WriteMode, () => Promise<void>>): Promise<void> =>
  (runs[mode as WriteMode] ??
    (() => {
      throw new DatasourceError("Unknown persist mode");
    }))();

export class DefaultWriteEnrichmentStrategy<T> implements IWriteEnrichmentStrategy<T> {
  protected skipUnloaded = false;
  readonly useTransaction: boolean;
  private readonly createStrategy: CreateEnrichmentStrategy<T>;
  private readonly deleteStrategy: DeleteEnrichmentStrategy<T>;
  private readonly updateStrategy: UpdateEnrichmentStrategy<T>;

  constructor(
    deleteStrategy: DeleteEnrichmentStrategy<T> = new CascadeDeleteEnrichmentStrategy<T>(),
    updateStrategy: UpdateEnrichmentStrategy<T> = new DefaultUpdateEnrichmentStrategy<T>(),
    createStrategy: CreateEnrichmentStrategy<T> = new DefaultCreateEnrichmentStrategy<T>(),
    useTransaction = false,
  ) {
    this.deleteStrategy = deleteStrategy;
    this.updateStrategy = updateStrategy;
    this.createStrategy = createStrategy;
    this.useTransaction = useTransaction;
  }

  persist = async (tree: IRepositoryTree, items: T[], mode: WriteMode): Promise<void> => {
    await this.walk(tree.parent.value, items as object[], mode);
  };

  private walk = async (host: object, parents: object[], mode: WriteMode): Promise<void> => {
    const entries = enrichmentsOf(host);
    if (!entries) return;
    for (const entry of entries) await this.persistEntry(entry, parents, mode);
  };

  private persistEntry = async (
    entry: StoredEnrichment,
    parents: object[],
    mode: WriteMode,
  ): Promise<void> => {
    const enrichment = entry.enrichment;
    const type = enrichment.type ?? EnrichmentPropertyType.Object;
    if (type === EnrichmentPropertyType.Custom)
      throw new DatasourceError(`Unsupported enrichment type "${type}"`);
    if (enrichment.association) {
      const writer = linkWriter(entry, enrichment.destinationProperty);
      for (const parent of parents) {
        if (this.skip(parent, enrichment.destinationProperty, mode)) continue;
        await dispatch(mode, {
          create: () => this.createStrategy.createLinked(writer, enrichment, parent),
          update: () => this.updateStrategy.updateLinked(writer, enrichment, parent),
          delete: () => this.deleteStrategy.deleteLinked(writer, enrichment, parent),
        });
      }
      return;
    }
    if (type === EnrichmentPropertyType.List && enrichment.relatedProperty) return;
    if (type !== EnrichmentPropertyType.List && type !== EnrichmentPropertyType.Dictionary) return;
    const writer = childWriter(entry, enrichment.destinationProperty);
    for (const parent of parents) {
      if (this.skip(parent, enrichment.destinationProperty, mode)) continue;
      await dispatch(mode, {
        create: () => this.createStrategy.createOwned(writer, enrichment, parent),
        update: () => this.updateStrategy.updateOwned(writer, enrichment, parent),
        delete: () => this.deleteStrategy.deleteOwned(writer, enrichment, parent, async () => {}),
      });
    }
  };

  private skip = (parent: object, into: string, mode: string): boolean =>
    this.skipUnloaded && mode !== "delete" && unloaded(parent, into);
}
