import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { ICRUDRepository } from "../ICRUDRepository.ts";
import type { Response } from "../Response.ts";
import type { IWhere } from "../IWhere.ts";

export class MockRepository<T = unknown> implements ICRUDRepository<T> {
  items: T[] = [];
  getSingleCalls: Array<IQuery | IWhere | undefined> = [];
  getCalls: Array<IQuery | IWhere | undefined> = [];
  addCalls: T[] = [];
  updateCalls: Array<{ item: T; byWhere?: IWhere }> = [];
  deleteCalls: Array<{ item: T; byWhere?: IWhere }> = [];
  getSingleError?: unknown;
  getError?: unknown;
  addError?: unknown;
  updateError?: unknown;
  deleteError?: unknown;
  addResponse: Response = { result: undefined, rowsAffected: 1 };
  updateResponse: Response = { result: undefined, rowsAffected: 1 };
  deleteResponse: Response = { result: undefined, rowsAffected: 1 };

  constructor(_connection: IConnection) {}

  async getSingle(query?: IQuery | IWhere): Promise<T | undefined> {
    this.getSingleCalls.push(query);
    if (this.getSingleError) {
      throw this.getSingleError;
    }
    return this.items[0];
  }

  async *get(query?: IQuery | IWhere): AsyncIterable<T> {
    this.getCalls.push(query);
    if (this.getError) {
      throw this.getError;
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
