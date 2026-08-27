import {
  ArrayReader,
  identityFilter,
  MiddlewareHost,
  type ICommand,
  type IReader,
  type IResponse,
  type Parameter,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";
import type { MemoryRecord } from "./store.ts";
import { matches, parametersToObject, parseFilter } from "./filter.ts";

export class MemoryCommand extends MiddlewareHost implements ICommand {
  constructor(readonly connection: MemoryConnection) {
    super();
  }

  async executeReader(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader> {
    return this.run(
      "executeReader",
      async () => {
        this.connection.ensureOpen();
        const filter = {
          ...parseFilter(text),
          ...parametersToObject(parameters),
        };
        const rows = this.connection
          .getCollection()
          .filter((row) => matches(row, filter));
        return new ArrayReader(rows);
      },
      { text, parameters },
    );
  }

  async execute(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IResponse> {
    return this.run(
      "execute",
      async () => {
        this.connection.ensureOpen();
        const op = (text ?? "").trim().toLowerCase();
        const obj = parametersToObject(parameters);
        if (op === "insert") {
          this.connection.getCollection().push({ ...obj });
          return { result: obj, rowsAffected: 1 };
        }
        if (op === "update") {
          const whereText =
            typeof obj.$where === "string" ? obj.$where : undefined;
          const next = { ...obj };
          delete next.$where;
          const filter = whereText ? parseFilter(whereText) : identityFilter(next);
          return this.applyUpdate(next, filter);
        }
        if (op === "delete") {
          const filter =
            Object.keys(obj).length === 0 ? {} : identityFilter(obj);
          return this.applyDelete(filter);
        }
        throw new Error(`Unsupported Memory command: ${text ?? ""}`);
      },
      { text, parameters },
    );
  }

  private applyUpdate(
    obj: MemoryRecord,
    filter: MemoryRecord,
  ): IResponse {
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

  private applyDelete(filter: MemoryRecord): IResponse {
    const collection = this.connection.getCollection();
    const remaining = collection.filter((row) => !matches(row, filter));
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    return { result: undefined, rowsAffected };
  }
}
