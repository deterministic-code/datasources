import type { ICRUDRepository } from "./ICRUDRepository.ts";
import type { IReadonlyEnrichmentStrategy } from "./IReadonlyEnrichmentStrategy.ts";
import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IEnriched<T> {
  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this;
  addLinked<TRelated extends object, TLink extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    link: ICRUDRepository<TLink>,
    parent: keyof TLink & string,
    related: keyof TLink & string,
    strategy?: IReadonlyEnrichmentStrategy<T & object>,
  ): this;
  enrich(items: T[]): Promise<void>;
}
