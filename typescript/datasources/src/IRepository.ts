import type { Constructable } from "./converters/Constructable.ts";

export interface IRepository<T> {
  readonly type: Constructable<T>;
}
