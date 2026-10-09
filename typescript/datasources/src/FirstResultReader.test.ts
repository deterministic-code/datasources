import { describe, expect, it } from "vitest";
import { FirstResultReader, StreamingReader } from "./index.ts";

describe("FirstResultReader", () => {
  it("replays the first set then advances", async () => {
    const sets = [[{ id: 1 }, { id: 2 }], [{ id: 3 }]];
    let set = 0;
    let row = 0;
    let closed = false;
    const live = new StreamingReader(
      async () => (row >= (sets[set]?.length ?? 0) ? undefined : sets[set]![row++]!),
      async () => {
        set += 1;
        row = 0;
        return set < sets.length;
      },
      async () => {
        closed = true;
      },
    );
    const { value: reader, rows } = await FirstResultReader.capture(live);
    expect(rows).toEqual([{ id: 1 }, { id: 2 }]);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 1 });
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 2 });
    expect(await reader.next()).toBe(false);
    expect(await reader.nextResult()).toBe(true);
    expect(await reader.next()).toBe(true);
    expect(reader.current).toEqual({ id: 3 });
    await reader.dispose();
    expect(closed).toBe(true);
    const empty = await FirstResultReader.capture(
      new StreamingReader(
        async () => undefined,
        async () => false,
        async () => {},
      ),
    );
    expect(await Array.fromAsync(empty.value.read())).toEqual([]);
    await empty.value[Symbol.asyncDispose]();
  });
});
