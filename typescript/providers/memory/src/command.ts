import {
  ArrayReader,
  CommandMiddlewareHost,
  identityFilter,
  type Cells,
  type ICommand,
  type IReader,
  type Response,
  type Parameter,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";
import type { MemoryRecord } from "./store.ts";
import { matches, parametersToObject, parseFilter } from "./filter.ts";

export class MemoryCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: MemoryConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const filter = {
        ...parseFilter(query),
        ...parametersToObject(params),
      };
      const rows = this.connection
        .getCollection()
        .filter((row) => matches(row, filter));
      return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
    });
  }

  async execute(
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
        this.connection.ensureOpen();
        const op = query.trim().toLowerCase();
        const obj = parametersToObject(params);
        if (op === "insert") {
          this.connection.getCollection().push({ ...obj });
          return { value: { result: obj, rowsAffected: 1, parameters: params }, rows: [] };
        }
        if (op === "update") {
          const whereText =
            typeof obj.$where === "string" ? obj.$where : undefined;
          const next = { ...obj };
          delete next.$where;
          const filter = whereText ? parseFilter(whereText) : identityFilter(next);
          return { value: { ...this.applyUpdate(next, filter), parameters: params }, rows: [] };
        }
        if (op === "delete") {
          const filter =
            Object.keys(obj).length === 0 ? {} : identityFilter(obj);
          return { value: { ...this.applyDelete(filter), parameters: params }, rows: [] };
        }
        throw new Error(`Unsupported Memory command: ${query}`);
    });
  }

  private applyUpdate(
    obj: MemoryRecord,
    filter: MemoryRecord,
  ): Response {
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(collection[i], filter)) {
        collection[i] = { ...collection[i], ...obj };
        rowsAffected += 1;
      }
    }
    return { result: obj, rowsAffected };
  }

  private applyDelete(filter: MemoryRecord): Response {
    const collection = this.connection.getCollection();
    const remaining = collection.filter((row) => !matches(row, filter));
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    return { result: undefined, rowsAffected };
  }
}
