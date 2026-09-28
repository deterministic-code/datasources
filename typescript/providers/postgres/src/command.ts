import {
  ArrayReader,
  CommandMiddlewareHost,
  ParameterDirection,
  responseOf,
  splitStatements,
  statementReader,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
  type Response,
} from "@datasources/core";
import { PostgresStatementFactory } from "./PostgresStatementFactory.ts";
import { bindParameters, toPostgres, withReturning } from "./bind.ts";
import { PostgresDriver } from "./PostgresDriver.ts";
import type { PostgresConnection } from "./connection.ts";
import type { PostgresExecuteResult } from "./i-postgres-driver.ts";
import type { Scalar } from "./scalar.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");
const isInsert = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("INSERT");
const isCall = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("CALL");
const hasOutput = (parameters?: Parameter[]) =>
  parameters?.some(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? false;

const firstRow = async (rows: AsyncIterable<Cells>): Promise<Cells | undefined> => {
  for await (const row of rows) return row;
  return undefined;
};

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
      const statements = splitStatements(query);
      if (statements.length === 0) return { value: new ArrayReader([]), rows: [] };
      const bound = bindParameters(params);
      return {
        value: statementReader(statements, (sql) => this.query(sql, bound)),
        rows: [],
      };
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReader(
      new PostgresStatementFactory().transform(statement).getText(),
      parameters,
    );
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    return this.execute(new PostgresStatementFactory().transform(statement).getText(), parameters);
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
      const row = await firstRow(this.query(rewritten, params));
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
    const id = queried.rows[0]?.id ?? null;
    return {
      rowsAffected: queried.rowCount ?? queried.rows.length,
      result: isInsert(sql) ? (new PostgresDriver().generatedId(id) as Scalar) : id,
      parameters,
    };
  }

  private query(sql: string, params?: Record<string, Scalar>): AsyncIterable<Cells> {
    const { text, values } = toPostgres(sql, params);
    return this.connection.require().rows(text, values);
  }
}
