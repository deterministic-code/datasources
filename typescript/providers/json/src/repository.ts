import {
  identityFilter,
  readerToAsyncIterable,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
} from "@datasources/core";
import type { JsonConnection } from "./connection.ts";
import { JsonCommand } from "./command.ts";
import { asRecord, matches, parseFilter } from "./filter.ts";

export class JsonRepository<T = unknown> implements IRepository<T> {
  constructor(private readonly connection: JsonConnection) {}

  async getSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    const reader = await new JsonCommand(this.connection).executeReader(
      query?.getText() ?? "",
    );
    if (await reader.next()) {
      return reader.current as T | undefined;
    }
    return undefined;
  }

  get(query?: IQuery | IWhere): AsyncIterable<T> {
    const command = new JsonCommand(this.connection);
    const text = query?.getText() ?? "";
    return {
      [Symbol.asyncIterator]: async function* () {
        const reader = await command.executeReader(text);
        yield* readerToAsyncIterable(reader as never);
      },
    };
  }

  async add(item: T): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(item);
    this.connection.getCollection().push(record);
    await this.connection.persist();
    return { result: item, rowsAffected: 1 };
  }

  async update(item: T, byWhere?: IWhere): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(item);
    const filter = byWhere
      ? parseFilter(byWhere.getText())
      : identityFilter(record);
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(asRecord(collection[i]), filter)) {
        collection[i] = { ...asRecord(collection[i]), ...record };
        rowsAffected += 1;
      }
    }
    await this.connection.persist();
    return { result: item, rowsAffected };
  }

  async delete(item: T, byWhere?: IWhere): Promise<IResponse> {
    this.connection.ensureOpen();
    const record = asRecord(item);
    const filter = byWhere
      ? parseFilter(byWhere.getText())
      : identityFilter(record);
    const collection = this.connection.getCollection();
    const remaining = collection.filter(
      (row) => !matches(asRecord(row), filter),
    );
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    await this.connection.persist();
    return { result: undefined, rowsAffected };
  }
}
