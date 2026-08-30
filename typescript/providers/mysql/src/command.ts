import {
  ArrayReader,
  CommandMiddlewareHost,
  type ICommand,
  type IReader,
  type IResponse,
  type Parameter,
} from "@datasources/core";
import type { MysqlConnection } from "./connection.ts";
import { bindParameters } from "./driver.ts";

export class MysqlCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: MysqlConnection) {
    super();
  }

  async executeReader(
    text: string,
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
    text: string,
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
