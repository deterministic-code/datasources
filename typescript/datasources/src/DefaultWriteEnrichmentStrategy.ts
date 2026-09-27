import { CascadeDeleteEnrichmentStrategy } from "./CascadeDeleteEnrichmentStrategy.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { DefaultUpdateEnrichmentStrategy } from "./DefaultUpdateEnrichmentStrategy.ts";
import type { ICRUDRepository } from "./ICRUDRepository.ts";
import { enrichmentsOf } from "./ReadonlyRepository.ts";
import type { IRepositoryTree } from "./IRepositoryTree.ts";
import type { IWriteEnrichmentStrategy } from "./IWriteEnrichmentStrategy.ts";
import type { StoredEnrichment } from "./StoredEnrichment.ts";
import type { Scalar } from "./builders/Scalar.ts";
import type { UpdateEnrichmentStrategy } from "./UpdateEnrichmentStrategy.ts";
import { createChildren, createLinks, getCell } from "./UpdateEnrichmentStrategy.ts";
import type { WriteMode } from "./WriteMode.ts";

type Props = Record<string, Scalar | object>;

const unloaded = (parent: object, into: string): boolean => {
  const value = getCell(parent, into);
  return value === undefined || value === null;
};

export const strip = (host: object, item: object): object => {
  const entries = enrichmentsOf(host);
  if (!entries) return item;
  const copy = { ...item } as Props;
  for (const entry of entries) delete copy[entry.enrichment.into];
  return copy;
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

  constructor(
    private readonly deleteStrategy = new CascadeDeleteEnrichmentStrategy<T>(),
    private readonly updateStrategy: UpdateEnrichmentStrategy<T> = new DefaultUpdateEnrichmentStrategy<T>(),
    useTransaction = false,
  ) {
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
    if (enrichment.association) {
      const writer = linkWriter(entry, enrichment.into);
      for (const parent of parents) {
        if (this.skip(parent, enrichment.into, mode)) continue;
        await dispatch(mode, {
          create: () => createLinks(writer, enrichment, parent),
          update: () => this.updateStrategy.updateLinked(entry, enrichment, parent),
          delete: () => this.deleteStrategy.deleteLinked(entry, enrichment, parent),
        });
      }
      return;
    }
    if (!enrichment.many || enrichment.select) return;
    const writer = childWriter(entry, enrichment.into);
    for (const parent of parents) {
      if (this.skip(parent, enrichment.into, mode)) continue;
      await dispatch(mode, {
        create: () => createChildren(writer, enrichment, parent),
        update: () => this.updateStrategy.updateOwned(entry, enrichment, parent),
        delete: () => this.deleteStrategy.deleteOwned(entry, enrichment, parent, async () => {}),
      });
    }
  };

  private skip = (parent: object, into: string, mode: string): boolean =>
    this.skipUnloaded && mode !== "delete" && unloaded(parent, into);
}
