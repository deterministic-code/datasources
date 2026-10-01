import type { IReader } from "../IReader.ts";
import { readerToAsyncIterable } from "../readerToAsyncIterable.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubReader<T = unknown> implements IReader<T> {
  current: T | undefined = undefined;

  constructor(readonly provider: string) {}

  async next(): Promise<boolean> {
    notImplemented(this.provider);
  }

  read(): AsyncIterable<T> {
    return readerToAsyncIterable(this);
  }

  async nextResult(): Promise<boolean> {
    notImplemented(this.provider);
  }

  async dispose(): Promise<void> {}

  [Symbol.asyncDispose] = (): Promise<void> => this.dispose();
}
