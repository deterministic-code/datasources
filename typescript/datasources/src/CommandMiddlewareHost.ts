import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Schema } from "./Schema.ts";
import type { IReader } from "./IReader.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";
import type { Value, Cells } from "./converters/Value.ts";
import type { IStatement } from "./statements/IStatement.ts";
import { DefaultStatementTextRenderer } from "./statements/DefaultStatementTextRenderer.ts";
import { Native } from "./statements/Native.ts";
import { StatementParser } from "./statements/StatementParser.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { Response } from "./Response.ts";

const asError = (error: unknown): Error =>
  error instanceof Error ? error : new Error(String(error));

type ExecuteBody = { result: Value; rowsAffected: number };

const isResponse = (value: object): value is ExecuteBody =>
  "rowsAffected" in value && typeof (value as ExecuteBody).rowsAffected === "number";

const logged = <T>(value: T, rows: readonly Row[]): readonly Row[] =>
  rows.length > 0 || value === null || typeof value !== "object" || !isResponse(value)
    ? rows
    : [{ result: value.result, rowsAffected: value.rowsAffected }];

export class CommandMiddlewareHost {
  private readonly middlewares: ICommandMiddleware[] = [];
  schema: Schema | null = null;

  addMiddleware(middleware: ICommandMiddleware): this {
    this.middlewares.push(middleware);
    return this;
  }

  execute(text: string, parameters?: Parameter[]): Promise<Response> {
    if (!text.trim()) throw new Error("empty command");
    return this.executeStatement(StatementParser.parse(text), parameters);
  }

  executeReader(text: string, parameters?: Parameter[]): Promise<IReader<Cells>> {
    if (!text.trim()) throw new Error("empty command");
    return this.executeReaderStatement(StatementParser.parse(text), parameters);
  }

  executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    return this.executeNative(
      new Native(DefaultStatementTextRenderer.instance.render(statement)),
      parameters,
    );
  }

  protected readsCell = (statement: IStatement): boolean =>
    ({
      SelectStatement: () => true,
      ReturnStatement: () => true,
    }[statement.constructor.name] ?? (() => false))();

  protected async firstCell(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
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

  executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(
      new Native(DefaultStatementTextRenderer.instance.render(statement)),
      parameters,
    );
  }

  executeNative(_text: Native, _parameters?: Parameter[]): Promise<Response> {
    throw new Error("executeNative");
  }

  executeReaderNative(_text: Native, _parameters?: Parameter[]): Promise<IReader<Cells>> {
    throw new Error("executeReaderNative");
  }

  async run<T>(
    text: string,
    params: Parameter[] | undefined,
    fn: (
      text: string,
      params?: Parameter[],
    ) => Promise<{ value: T; rows: readonly Row[] }>,
  ): Promise<T> {
    let q = text.trim();
    let p = params;
    for (const middleware of this.middlewares) {
      const rewritten = await middleware.beforeCommand(q, p);
      if (rewritten) {
        q = rewritten.text;
        p = rewritten.params as Parameter[] | undefined;
      }
    }

    const start = performance.now();
    try {
      const { value, rows } = await fn(q, p);
      const elapsedMs = performance.now() - start;
      for (const middleware of this.middlewares) {
        await middleware.afterCommand(q, p, logged(value, rows), elapsedMs);
      }
      return value;
    } catch (error) {
      const elapsedMs = performance.now() - start;
      for (const middleware of this.middlewares) {
        await middleware.afterCommand(q, p, [], elapsedMs, asError(error));
      }
      throw error;
    }
  }
}
