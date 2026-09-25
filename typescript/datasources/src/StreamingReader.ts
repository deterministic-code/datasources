import type { Cells } from "./converters/Cell.ts";
import type { IReader } from "./IReader.ts";

export class StreamingReader implements IReader<Cells> {
  current: Cells | undefined = undefined;
  private closed = false;

  constructor(
    private readonly read: () => Promise<Cells | undefined>,
    private readonly advance: () => Promise<boolean>,
    private readonly close: () => Promise<void> = async () => {},
  ) {}

  async next(): Promise<boolean> {
    this.current = await this.read();
    return this.current !== undefined;
  }

  async nextResult(): Promise<boolean> {
    this.current = undefined;
    return this.advance();
  }

  async dispose(): Promise<void> {
    if (this.closed) return;
    this.closed = true;
    await this.close();
  }
}

export const statementReader = (
  statements: readonly string[],
  load: (sql: string) => AsyncIterable<Cells>,
): StreamingReader => {
  let index = 0;
  let rows: AsyncIterator<Cells> | undefined;
  const closeRows = async () => {
    await rows?.return?.();
    rows = undefined;
  };
  return new StreamingReader(
    async () => {
      if (index >= statements.length) return undefined;
      const sql = statements[index];
      if (!sql) return undefined;
      rows ??= load(sql)[Symbol.asyncIterator]();
      const step = await rows.next();
      return step.done ? undefined : step.value;
    },
    async () => {
      await closeRows();
      index += 1;
      return index < statements.length;
    },
    closeRows,
  );
};
