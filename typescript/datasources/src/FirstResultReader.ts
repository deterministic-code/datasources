import type { Cells } from "./converters/Cell.ts";
import type { IReader } from "./IReader.ts";
import { readerToAsyncIterable } from "./readerToAsyncIterable.ts";

export class FirstResultReader implements IReader<Cells> {
  current: Cells | undefined = undefined;
  private index = -1;
  private firstSet = true;

  private constructor(
    private readonly first: readonly Cells[],
    private readonly live: IReader<Cells>,
  ) {}

  static async capture(live: IReader<Cells>): Promise<{ value: IReader<Cells>; rows: Cells[] }> {
    const rows: Cells[] = [];
    while (await live.next()) if (live.current != null) rows.push(live.current);
    return { value: new FirstResultReader(rows, live), rows };
  }

  async next(): Promise<boolean> {
    if (!this.firstSet) {
      const moved = await this.live.next();
      this.current = this.live.current;
      return moved;
    }
    this.index += 1;
    if (this.index < this.first.length) {
      this.current = this.first[this.index];
      return true;
    }
    this.current = undefined;
    return false;
  }

  async nextResult(): Promise<boolean> {
    this.firstSet = false;
    this.current = undefined;
    return this.live.nextResult();
  }

  read(): AsyncIterable<Cells> {
    return readerToAsyncIterable(this);
  }

  dispose = (): Promise<void> => this.live.dispose();

  [Symbol.asyncDispose] = (): Promise<void> => this.dispose();
}
