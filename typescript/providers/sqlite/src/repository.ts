import {
  identityWhereText,
  readerToAsyncIterable,
  type IQuery,
  type IRepository,
  type IResponse,
  type IWhere,
  type Parameter,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";
import { SqliteCommand } from "./command.ts";

export interface SqliteRepositoryOptions {
  table?: string;
}

export class SqliteRepository<T = unknown> implements IRepository<T> {
  readonly table: string;

  constructor(
    readonly connection: SqliteConnection,
    options: SqliteRepositoryOptions = {},
  ) {
    this.table = options.table ?? "items";
  }

  async getSingle(query?: IQuery): Promise<T | undefined> {
    const sql = this.selectSql(query, true);
    const reader = await new SqliteCommand(this.connection).executeReader(
      sql,
      this.queryParameters(query),
    );
    if (await reader.next()) {
      return reader.current as T | undefined;
    }
    return undefined;
  }

  get(query?: IQuery): AsyncIterable<T> {
    const command = new SqliteCommand(this.connection);
    const sql = this.selectSql(query, false);
    const parameters = this.queryParameters(query);
    return {
      [Symbol.asyncIterator]: async function* () {
        const reader = await command.executeReader(sql, parameters);
        yield* readerToAsyncIterable(reader as never);
      },
    };
  }

  async add(obj: T): Promise<IResponse> {
    const record = this.asRecord(obj);
    const keys = Object.keys(record);
    const sql =
      keys.length === 0
        ? `INSERT INTO ${this.table} DEFAULT VALUES`
        : `INSERT INTO ${this.table} (${keys.join(", ")}) VALUES (${keys
            .map((key) => `$${key}`)
            .join(", ")})`;
    return new SqliteCommand(this.connection).execute(
      sql,
      this.toParameters(record),
    );
  }

  async update(obj: T, byWhere?: IWhere): Promise<IResponse> {
    const record = this.asRecord(obj);
    const keys = Object.keys(record);
    const assignments =
      keys.length === 0
        ? "id = id"
        : keys.map((key) => `${key} = $${key}`).join(", ");
    const where = byWhere?.getText() ?? identityWhereText(record);
    const sql = `UPDATE ${this.table} SET ${assignments} WHERE ${where}`;
    return new SqliteCommand(this.connection).execute(
      sql,
      this.toParameters(record),
    );
  }

  async delete(obj: T, byWhere?: IWhere): Promise<IResponse> {
    const record = this.asRecord(obj);
    const where = byWhere?.getText() ?? identityWhereText(record);
    const sql = `DELETE FROM ${this.table} WHERE ${where}`;
    return new SqliteCommand(this.connection).execute(
      sql,
      this.toParameters(record),
    );
  }

  private selectSql(query: IQuery | undefined, single: boolean): string {
    const where = query ? ` WHERE ${query.getText()}` : "";
    const limit = single ? " LIMIT 1" : "";
    return `SELECT * FROM ${this.table}${where}${limit}`;
  }

  private queryParameters(query?: IQuery): Parameter[] | undefined {
    if (!query) {
      return undefined;
    }
    const extra = (query as { parameters?: Parameter[] }).parameters;
    return extra;
  }

  private asRecord(obj: T): Record<string, unknown> {
    if (obj === null || typeof obj !== "object") {
      throw new Error("SQLite repository objects must be plain objects");
    }
    return obj as Record<string, unknown>;
  }

  private toParameters(record: Record<string, unknown>): Parameter[] {
    return Object.entries(record).map(([name, value]) => ({ name, value }));
  }
}
