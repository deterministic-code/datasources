import {
  ArrayReader,
  CommandMiddlewareHost,
  parametersToRecord,
  responseOf,
  splitStatements,
  StreamingReader,
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
      const statements = splitStatements(query);
      if (statements.length === 0) return { value: new ArrayReader([]), rows: [] };
      const paramsBound = bound(params);
      let index = 0;
      let iter = this.iterate(statements[0] as string, paramsBound);
      return {
        value: new StreamingReader(
          async () => {
            const step = iter.next();
            return step.done ? undefined : step.value;
          },
          async () => {
            iter.return();
            index += 1;
            const sql = statements[index];
            if (!sql) return false;
            iter = this.iterate(sql, paramsBound);
            return true;
          },
          async () => {
            iter.return();
          },
        ),
        rows: [],
      };
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

  private iterate(sql: string, params: Record<string, Scalar>): IterableIterator<Cells> & {
    return(): IteratorResult<Cells>;
  } {
    const iter = this.connection.require().prepare(sql).iterate(params);
    const stop = iter.return;
    return {
      next: () => iter.next(),
      return: () => (stop ? stop.call(iter) : { done: true, value: undefined }),
    };
  }
}
