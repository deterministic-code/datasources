import {
  identityFilter,
  readerToAsyncIterable,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";
import { MemoryCommand } from "./command.ts";
import { asRecord, matches, parseFilter } from "./filter.ts";

export class MemoryRepository<T = unknown> implements IRepository<T> {
  constructor(readonly connection: MemoryConnection) {}

  async getSingle(query?: IQuery): Promise<T | undefined> {
    const reader = await new MemoryCommand(this.connection).executeReader(
      query?.getText() ?? null,
    );
    if (await reader.next()) {
      return reader.current as T | undefined;
    }
    return undefined;
  }

  get(query?: IQuery): AsyncIterable<T> {
    const command = new MemoryCommand(this.connection);
    const text = query?.getText() ?? null;
    return {
      [Symbol.asyncIterator]: async function* () {
        const reader = await command.executeReader(text);
        yield* readerToAsyncIterable(reader as never);
      },
    };
  }

  async add(obj: T): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(obj);
    this.connection.getCollection().push({ ...record });
    return { result: obj, rowsAffected: 1 };
  }

  async update(obj: T, byWhere?: IWhere): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(obj);
    const filter = byWhere
      ? parseFilter(byWhere.getText())
      : identityFilter(record);
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(collection[i], filter)) {
        collection[i] = { ...collection[i], ...record };
        rowsAffected += 1;
      }
    }
    return { result: obj, rowsAffected };
  }

  async delete(obj: T, byWhere?: IWhere): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(obj);
    const filter = byWhere
      ? parseFilter(byWhere.getText())
      : identityFilter(record);
    const collection = this.connection.getCollection();
    const remaining = collection.filter((row) => !matches(row, filter));
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    return { result: undefined, rowsAffected };
  }
}
