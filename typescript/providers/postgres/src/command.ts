import {
  ArrayReader,
  CommandMiddlewareHost,
  FirstResultReader,
  DatasourceError,
  Native,
  ParameterDirection,
  ReturnStatement,
  SelectStatement,
  StatementTextRenderer,
  Response,
  splitStatements,
  statementUsesParameter,
  statementReader,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
} from "@datasources/core";
import { bindParameters, toPostgres } from "./bind.ts";
import type { PostgresConnection } from "./connection.ts";
import type { PostgresExecuteResult } from "./i-postgres-driver.ts";
import type { Scalar } from "./scalar.ts";
import { PostgresStatementRenderer } from "./PostgresStatementRenderer.ts";

export class PostgresCommand extends CommandMiddlewareHost implements ICommand {
  private readonly statements: StatementTextRenderer;

  constructor(readonly connection: PostgresConnection, statements?: StatementTextRenderer) {
    super();
    this.statements = statements ?? new StatementTextRenderer(t => new PostgresStatementRenderer(t));
  }

  async executeReaderNative(
    text: Native,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new ArrayReader([]), rows: [] };
      this.connection.ensureOpen();
      const statements = splitStatements(query);
      if (statements.length === 0) return { value: new ArrayReader([]), rows: [] };
      const bound = bindParameters(params);
      return FirstResultReader.capture(statementReader(statements, (sql) => this.query(sql, bound)));
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(new Native(this.statements.render(statement)), parameters);
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    return statement instanceof SelectStatement || statement instanceof ReturnStatement
      ? this.firstCell(statement, parameters)
      : this.executeNative(new Native(this.statements.render(statement)), parameters);
  }

  private async firstCell(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    const reader = await this.executeReaderStatement(statement, parameters);
    try {
      if (!(await reader.next())) return new Response(null, 0, parameters);
      const row = reader.current;
      /* v8 ignore next */
      if (!row) throw new DatasourceError("Reader current must be a row");
      const values = Object.values(row);
      return new Response(values.length === 0 ? null : values[0], 1, parameters);
    } finally {
      await reader.dispose();
    }
  }

  async executeNative(text: Native, parameters?: Parameter[]): Promise<Response> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim())
        return { value: new Response(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      const named = bindParameters(params);
      let rowsAffected = 0;
      let parameters = params;
      for (const part of splitStatements(query)) {
        const used = Object.fromEntries(
          Object.entries(named).filter(([name]) => statementUsesParameter(part, name)),
        );
        const result = await this.runSql(part, used, params);
        rowsAffected += result.rowsAffected;
        parameters = result.parameters ?? params;
      }
      return {
        value: new Response(null, rowsAffected, parameters),
        rows: [],
      };
    });
  }

  private async runSql(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<PostgresExecuteResult> {
    const { text, values } = toPostgres(sql, params);
    const queried = await this.connection.require().query(text, values);
    const row = queried.rows[0];
    return {
      rowsAffected: queried.rowCount ?? queried.rows.length,
      result: null,
      parameters: parameters?.map((parameter) =>
        parameter.direction &&
        parameter.direction !== ParameterDirection.Input &&
        row &&
        parameter.name in row
          ? { ...parameter, value: row[parameter.name] as Scalar }
          : parameter,
      ),
    };
  }

  private query(sql: string, params?: Record<string, Scalar>): AsyncIterable<Cells> {
    const { text, values } = toPostgres(sql, params);
    return this.connection.require().rows(text, values);
  }
}
