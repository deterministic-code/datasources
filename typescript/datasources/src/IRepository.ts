import type { Constructable } from "./converters/Constructable.ts";
import type { Schema } from "./Schema.ts";

export interface IRepository<T> {
  readonly type: Constructable<T>;
  readonly schema: Schema;
}
