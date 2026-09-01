import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { IRepository } from "../IRepository.ts";
import type { IResponse } from "../IResponse.ts";
import type { IWhere } from "../IWhere.ts";

export class MockRepository<T = unknown> implements IRepository<T> {
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
  addResponse: IResponse = { result: undefined, rowsAffected: 1 };
  updateResponse: IResponse = { result: undefined, rowsAffected: 1 };
  deleteResponse: IResponse = { result: undefined, rowsAffected: 1 };

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

  async add(item: T): Promise<IResponse> {
    this.addCalls.push(item);
    if (this.addError) {
      throw this.addError;
    }
    this.items.push(item);
    this.addResponse = { result: item, rowsAffected: 1 };
    return this.addResponse;
  }

  async update(item: T, byWhere?: IWhere): Promise<IResponse> {
    this.updateCalls.push({ item, byWhere });
    if (this.updateError) {
      throw this.updateError;
    }
    return this.updateResponse;
  }

  async delete(item: T, byWhere?: IWhere): Promise<IResponse> {
    this.deleteCalls.push({ item, byWhere });
    if (this.deleteError) {
      throw this.deleteError;
    }
    return this.deleteResponse;
  }
}
