import type { IReader } from "./IReader.ts";

export class ArrayReader<T = unknown> implements IReader<T> {
  private index = -1;
  current: T | undefined = undefined;

  constructor(private readonly rows: readonly T[]) {}

  async next(): Promise<boolean> {
    this.index += 1;
    if (this.index < this.rows.length) {
      this.current = this.rows[this.index];
      return true;
    }
    this.current = undefined;
    return false;
  }

  async nextResult(): Promise<boolean> {
    this.current = undefined;
    return false;
  }

  async dispose(): Promise<void> {}

  [Symbol.asyncDispose] = (): Promise<void> => this.dispose();
}
