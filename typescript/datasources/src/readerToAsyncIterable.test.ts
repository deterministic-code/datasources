import { describe, expect, it } from "vitest";
import { ArrayReader, collectAsync, readerToAsyncIterable, type IReader } from "./index.ts";

describe("readerToAsyncIterable", () => {
  it("converts a reader to an async iterable", async () => {
    const items = await collectAsync(
      readerToAsyncIterable(new ArrayReader(["a", "b"])),
    );
    expect(items).toEqual(["a", "b"]);
    expect(await collectAsync(readerToAsyncIterable(new ArrayReader([])))).toEqual(
      [],
    );
  });

  it("skips a defined-next row whose current is undefined", async () => {
    const reader: IReader<string> = {
      current: undefined,
      read() {
        return readerToAsyncIterable(this);
      },
      async nextResult() {
        return false;
      },
      async dispose() {},
      async [Symbol.asyncDispose]() {
        await this.dispose();
      },
      async next() {
        if (this.current === undefined && (this as { once?: boolean }).once) {
          return false;
        }
        (this as { once?: boolean }).once = true;
        this.current = undefined;
        return true;
      },
    };
    expect(await collectAsync(readerToAsyncIterable(reader))).toEqual([]);
  });
});
