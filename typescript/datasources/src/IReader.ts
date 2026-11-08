import type { Cells } from "./converters/Value.ts";

export interface IReader<T = Cells> {
  readonly current: T | undefined;
  next(): Promise<boolean>;
  nextResult(): Promise<boolean>;
  read(): AsyncIterable<T>;
  dispose(): Promise<void>;
  [Symbol.asyncDispose](): Promise<void>;
}
