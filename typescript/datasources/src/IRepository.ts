import type { Constructable } from "./converters/Constructable.ts";

export interface IRepository<T = unknown> {
  readonly type: Constructable<T>;
}
