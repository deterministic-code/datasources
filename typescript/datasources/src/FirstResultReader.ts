import type { Cells } from "./converters/Cell.ts";
import type { IReader } from "./IReader.ts";
import { readerToAsyncIterable } from "./readerToAsyncIterable.ts";

export class FirstResultReader implements IReader<Cells> {
  current: Cells | undefined = undefined;
  private set = 0;
  private index = -1;

  private constructor(private readonly sets: readonly (readonly Cells[])[]) {}

  static async capture(live: IReader<Cells>): Promise<{ value: IReader<Cells>; rows: Cells[] }> {
    const sets: Cells[][] = [];
    try {
      for (;;) {
        const rows: Cells[] = [];
        while (await live.next()) if (live.current != null) rows.push(live.current);
        sets.push(rows);
        if (!(await live.nextResult())) break;
      }
    } finally {
      await live.dispose();
    }
    return { value: new FirstResultReader(sets), rows: sets[0] };
  }

  async next(): Promise<boolean> {
    if (this.set >= this.sets.length) {
      this.current = undefined;
      return false;
    }
    this.index += 1;
    const rows = this.sets[this.set]!;
    if (this.index < rows.length) {
      this.current = rows[this.index];
      return true;
    }
    this.current = undefined;
    return false;
  }

  async nextResult(): Promise<boolean> {
    this.current = undefined;
    this.index = -1;
    this.set += 1;
    return this.set < this.sets.length;
  }

  read(): AsyncIterable<Cells> {
    return readerToAsyncIterable(this);
  }

  dispose = (): Promise<void> => Promise.resolve();

  [Symbol.asyncDispose] = (): Promise<void> => this.dispose();
}
