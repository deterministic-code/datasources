import {
  ArrayReader,
  MiddlewareHost,
  type ICommand,
  type IReader,
  type IResponse,
  type Parameter,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";
import { bindParameters } from "./driver.ts";

export class SqliteCommand extends MiddlewareHost implements ICommand {
  constructor(readonly connection: SqliteConnection) {
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
        const rows = await this.connection.driver.query(
          text ?? "",
          bindParameters(parameters),
        );
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
        const result = await this.connection.driver.execute(
          text ?? "",
          bindParameters(parameters),
        );
        return { result: result.result, rowsAffected: result.rowsAffected };
      },
      { text, parameters },
    );
  }
}
