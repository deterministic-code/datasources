import {
  ArrayReader,
  CommandMiddlewareHost,
  ParameterDirection,
  responseOf,
  type Cells,
  type ICommand,
  type IReader,
  type Parameter,
  type Response,
} from "@datasources/core";
import { bindParameters, toPostgres, withReturning } from "./bind.ts";
import type { PostgresConnection } from "./connection.ts";
import type { PostgresExecuteResult } from "./i-postgres-driver.ts";
import type { Scalar } from "./scalar.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");
const isCall = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("CALL");
const hasOutput = (parameters?: Parameter[]) =>
  parameters?.some(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? false;

export class PostgresCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: PostgresConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new ArrayReader([]), rows: [] };
      this.connection.ensureOpen();
      const rows = await this.query(query, bindParameters(params));
      return { value: new ArrayReader(rows), rows };
    });
  }

  async execute(
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      if (!query.trim())
        return { value: responseOf(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      const result = await this.runSql(query, bindParameters(params), params);
      return {
        value: responseOf(result.result, result.rowsAffected, result.parameters ?? params),
        rows: [],
      };
    });
  }

  private async runSql(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<PostgresExecuteResult> {
    const rewritten = withReturning(sql);
    if (isSelect(rewritten) || (isCall(rewritten) && hasOutput(parameters))) {
      const rows = await this.query(rewritten, params);
      const row = rows[0];
      return {
        rowsAffected: row ? 1 : 0,
        result: row ? Object.values(row)[0] ?? null : null,
        parameters: row && parameters
          ? parameters.map((parameter) =>
              parameter.direction &&
              parameter.direction !== ParameterDirection.Input &&
              parameter.name in row
                ? { ...parameter, value: row[parameter.name] }
                : parameter,
            )
          : parameters,
      };
    }
    const { text, values } = toPostgres(rewritten, params);
    const queried = await this.connection.require().query(text, values);
    return {
      rowsAffected: queried.rowCount ?? queried.rows.length,
      result: queried.rows[0]?.id ?? null,
      parameters,
    };
  }

  private async query(sql: string, params?: Record<string, Scalar>): Promise<Cells[]> {
    const { text, values } = toPostgres(sql, params);
    return (await this.connection.require().query(text, values)).rows;
  }
}
