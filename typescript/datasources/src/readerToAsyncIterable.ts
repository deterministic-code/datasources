import type { IReader } from "./IReader.ts";

export async function* readerToAsyncIterable<T>(
  reader: IReader<T>,
): AsyncIterable<T> {
  while (await reader.next()) {
    if (reader.current !== undefined) {
      yield reader.current;
    }
  }
}
