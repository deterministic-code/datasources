import type { IReader } from "../IReader.ts";

export class MockReader<T = unknown> implements IReader<T> {
  current: T | undefined = undefined;
  nextCalls = 0;
  nextError?: unknown;
  private index = -1;

  constructor(public rows: readonly T[] = []) {}

  async next(): Promise<boolean> {
    this.nextCalls += 1;
    if (this.nextError) {
      throw this.nextError;
    }
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
}
