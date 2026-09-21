import type { Constructable } from "../converters/Constructable.ts";
import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../statements/IQuery.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import { ReadonlyRepository, untyped } from "../ReadonlyRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../statements/IWhere.ts";

export class MockRepository<T = unknown> extends ReadonlyRepository<T> implements ICRUDRepository<T> {
  items: T[] = [];
  readSingleCalls: Array<IQuery | IWhere | undefined> = [];
  readCalls: Array<IQuery | IWhere | undefined> = [];
  addCalls: T[] = [];
  updateCalls: Array<{ item: T; byWhere?: IWhere }> = [];
  deleteCalls: Array<{ item: T; byWhere?: IWhere }> = [];
  readSingleError?: unknown;
  readError?: unknown;
  addError?: unknown;
  updateError?: unknown;
  deleteError?: unknown;
  addResponse: Response = { result: undefined, rowsAffected: 1 };
  updateResponse: Response = { result: undefined, rowsAffected: 1 };
  deleteResponse: Response = { result: undefined, rowsAffected: 1 };

  constructor(
    _connection: IConnection,
    type: Constructable<T> = untyped<T>(),
  ) {
    super(type);
  }

  async readSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    this.readSingleCalls.push(query);
    if (this.readSingleError) {
      throw this.readSingleError;
    }
    return this.items[0];
  }

  async *read(query?: IQuery | IWhere): AsyncIterable<T> {
    this.readCalls.push(query);
    if (this.readError) {
      throw this.readError;
    }
    for (const item of this.items) {
      yield item;
    }
  }

  async create(item: T): Promise<Response> {
    this.addCalls.push(item);
    if (this.addError) {
      throw this.addError;
    }
    this.items.push(item);
    this.addResponse = { result: item, rowsAffected: 1 };
    return this.addResponse;
  }

  async update(item: T, byWhere?: IWhere): Promise<Response> {
    this.updateCalls.push({ item, byWhere });
    if (this.updateError) {
      throw this.updateError;
    }
    return this.updateResponse;
  }

  async delete(item: T, byWhere?: IWhere): Promise<Response> {
    this.deleteCalls.push({ item, byWhere });
    if (this.deleteError) {
      throw this.deleteError;
    }
    return this.deleteResponse;
  }
}
