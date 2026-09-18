import {
  ArrayReader,
  CommandMiddlewareHost,
  type Cells,
  type ICommand,
  type IReader,
  type Response,
  type Parameter,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";
import { bindParameters } from "./driver.ts";

export class SqliteCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: SqliteConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const rows = await this.connection.driver.query(
        query,
        bindParameters(params),
      );
      return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
    });
  }

  async execute(
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const result = await this.connection.driver.execute(
        query,
        bindParameters(params),
      );
      return {
        value: { result: result.result, rowsAffected: result.rowsAffected },
        rows: [],
      };
    });
  }
}
