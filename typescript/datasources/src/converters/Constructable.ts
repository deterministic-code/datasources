import type { Cells } from "./Value.ts";

export type Constructable<T = Cells> = new () => T;
