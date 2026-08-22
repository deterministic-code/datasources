import type { IConnection } from "../IConnection.ts";
import type { IQuery } from "../IQuery.ts";
import type { IRepository } from "../IRepository.ts";
import type { IResponse } from "../IResponse.ts";
import type { IWhere } from "../IWhere.ts";

export class MockRepository<T = unknown> implements IRepository<T> {
  items: T[] = [];
  getSingleCalls: Array<IQuery | undefined> = [];
  getCalls: Array<IQuery | undefined> = [];
  addCalls: T[] = [];
  updateCalls: Array<{ obj: T; byWhere?: IWhere }> = [];
  deleteCalls: Array<{ obj: T; byWhere?: IWhere }> = [];
  getSingleError?: unknown;
  getError?: unknown;
  addError?: unknown;
  updateError?: unknown;
  deleteError?: unknown;
  addResponse: IResponse = { result: undefined, rowsAffected: 1 };
  updateResponse: IResponse = { result: undefined, rowsAffected: 1 };
  deleteResponse: IResponse = { result: undefined, rowsAffected: 1 };

  constructor(readonly connection: IConnection) {}

  async getSingle(query?: IQuery): Promise<T | undefined> {
    this.getSingleCalls.push(query);
    if (this.getSingleError) {
      throw this.getSingleError;
    }
    return this.items[0];
  }

  async *get(query?: IQuery): AsyncIterable<T> {
    this.getCalls.push(query);
    if (this.getError) {
      throw this.getError;
    }
    for (const item of this.items) {
      yield item;
    }
  }

  async add(obj: T): Promise<IResponse> {
    this.addCalls.push(obj);
    if (this.addError) {
      throw this.addError;
    }
    this.items.push(obj);
    this.addResponse = { result: obj, rowsAffected: 1 };
    return this.addResponse;
  }

  async update(obj: T, byWhere?: IWhere): Promise<IResponse> {
    this.updateCalls.push({ obj, byWhere });
    if (this.updateError) {
      throw this.updateError;
    }
    return this.updateResponse;
  }

  async delete(obj: T, byWhere?: IWhere): Promise<IResponse> {
    this.deleteCalls.push({ obj, byWhere });
    if (this.deleteError) {
      throw this.deleteError;
    }
    return this.deleteResponse;
  }
}
