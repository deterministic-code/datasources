import {
  ArrayReader,
  CommandMiddlewareHost,
  responseOf,
  type Cells,
  type ICommand,
  type IReader,
  type Response,
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
  ): Promise<IReader<Cells>> {
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
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const result = await this.connection.driver.execute(
        query,
        bindParameters(params),
        params,
      );
      return {
        value: responseOf(result.result, result.rowsAffected, result.parameters ?? params),
        rows: [],
      };
    });
  }
}
