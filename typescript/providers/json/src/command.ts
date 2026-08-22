import {
  ArrayReader,
  identityFilter,
  MiddlewareHost,
  type ICommand,
  type IReader,
  type IResponse,
  type Parameter,
} from "@datasources/core";
import type { JsonConnection } from "./connection.ts";
import {
  asRecord,
  matches,
  parametersToObject,
  parseFilter,
} from "./filter.ts";

export class JsonCommand extends MiddlewareHost implements ICommand {
  constructor(readonly connection: JsonConnection) {
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
          .filter((row) => matches(asRecord(row), filter));
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
          this.connection.getCollection().push(obj);
          await this.connection.persist();
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
        throw new Error(`Unsupported JSON command: ${text ?? ""}`);
      },
      { text, parameters },
    );
  }

  private async applyUpdate(
    obj: Record<string, unknown>,
    filter: Record<string, unknown>,
  ): Promise<IResponse> {
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(asRecord(collection[i]), filter)) {
        collection[i] = { ...asRecord(collection[i]), ...obj };
        rowsAffected += 1;
      }
    }
    await this.connection.persist();
    return { result: obj, rowsAffected };
  }

  private async applyDelete(
    filter: Record<string, unknown>,
  ): Promise<IResponse> {
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
