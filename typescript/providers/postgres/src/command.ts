import {
  ArrayReader,
  CommandMiddlewareHost,
  type ICommand,
  type IReader,
  type IResponse,
  type Parameter,
} from "@datasources/core";
import type { PostgresConnection } from "./connection.ts";
import { bindParameters } from "./driver.ts";

export class PostgresCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: PostgresConnection) {
    super();
  }

  async executeReader(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const rows = await this.connection.driver.query(
        query,
        bindParameters(params),
      );
      return { value: new ArrayReader(rows), rows };
    });
  }

  async execute(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IResponse> {
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
