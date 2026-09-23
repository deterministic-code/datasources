import {
  ArrayReader,
  CommandMiddlewareHost,
  parametersToRecord,
  responseOf,
  type Cells,
  type Scalar,
  type ICommand,
  type IReader,
  type Parameter,
  type Response,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";
import { reverse } from "./converters/SqliteRowTypeConverters.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");

const bound = (parameters?: Parameter[]): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  return Object.fromEntries(
    Object.entries(record).map(([name, value]) => [name, reverse(value)]),
  ) as Record<string, Scalar>;
};

export class SqliteCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: SqliteConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new ArrayReader([]), rows: [] };
      this.connection.ensureOpen();
      const rows = this.connection.require().prepare(query).all(bound(params));
      return { value: new ArrayReader(rows), rows };
    });
  }

  async execute(text: string, parameters?: Parameter[]): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      if (!query.trim()) return { value: responseOf(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      if (isSelect(query)) {
        const rows = this.connection.require().prepare(query).all(bound(params));
        const row = rows[0];
        return {
          value: responseOf(row ? Object.values(row)[0] : null, row ? 1 : 0, params),
          rows: [],
        };
      }
      const info = this.connection.require().prepare(query).run(bound(params));
      return {
        value: responseOf(info.lastInsertRowid, info.changes, params),
        rows: [],
      };
    });
  }
}
