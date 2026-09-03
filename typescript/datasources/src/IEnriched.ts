import type { IReadonlyRepository } from "./IReadonlyRepository.ts";

export interface IEnriched<T> {
  addEnriched<TRelated extends object>(
    into: keyof T & string,
    from: keyof T & string,
    via: IReadonlyRepository<TRelated>,
    by: keyof TRelated & string,
    select?: keyof TRelated & string,
    many?: boolean,
  ): this;
  enrich(items: T[]): Promise<T[]>;
}
