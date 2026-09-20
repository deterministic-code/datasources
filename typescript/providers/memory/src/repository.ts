import {
  identityFilter,
  readerToAsyncIterable,
  type Constructable,
  type IQuery,
  type ICRUDRepository,
  type Response,
  type IWhere,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";
import { MemoryCommand } from "./command.ts";
import { asRecord, matches, parseFilter } from "./filter.ts";

export class MemoryRepository<T = unknown> implements ICRUDRepository<T> {
  constructor(
    private readonly connection: MemoryConnection,
    readonly type: Constructable<T> = Object as never as Constructable<T>,
  ) {}

  async readSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    const reader = await new MemoryCommand(this.connection).executeReader(
      query?.getText() ?? "",
    );
    if (await reader.next()) {
      return reader.current as T | undefined;
    }
    return undefined;
  }

  read(query?: IQuery | IWhere): AsyncIterable<T> {
    const command = new MemoryCommand(this.connection);
    const text = query?.getText() ?? "";
    return {
      [Symbol.asyncIterator]: async function* () {
        const reader = await command.executeReader(text);
        yield* readerToAsyncIterable(reader as never);
      },
    };
  }

  async create(item: T): Promise<Response> {
    this.connection.ensureOpen();
    const record = asRecord(item);
    this.connection.getCollection().push({ ...record });
    return { result: item, rowsAffected: 1 };
  }

  async update(item: T, byWhere?: IWhere): Promise<Response> {
    this.connection.ensureOpen();
    const record = asRecord(item);
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
    return { result: item, rowsAffected };
  }

  async delete(item: T, byWhere?: IWhere): Promise<Response> {
    this.connection.ensureOpen();
    const record = asRecord(item);
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
