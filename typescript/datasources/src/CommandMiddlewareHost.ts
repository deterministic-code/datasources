import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { IReader } from "./IReader.ts";
import type { Parameter } from "./Parameter.ts";
import type { Response } from "./Response.ts";
import type { Row } from "./Row.ts";
import type { Cell, Cells } from "./converters/Cell.ts";
import type { IStatement } from "./statements/IStatement.ts";
import { DefaultStatementTextRenderer } from "./statements/DefaultStatementTextRenderer.ts";
import { Native } from "./statements/Native.ts";
import { StatementParser } from "./statements/StatementParser.ts";

const asError = (error: unknown): Error =>
  error instanceof Error ? error : new Error(String(error));

type ExecuteBody = { result: Cell; rowsAffected: number };

const resultOf = (value: Cell | object | null): Cell =>
  value instanceof Date ||
  value instanceof Uint8Array ||
  value === null ||
  typeof value === "string" ||
  typeof value === "number" ||
  typeof value === "boolean" ||
  typeof value === "bigint"
    ? value
    : String(value);

const isResponse = (value: object): value is ExecuteBody =>
  "rowsAffected" in value && typeof (value as ExecuteBody).rowsAffected === "number";

const logged = <T>(value: T, rows: readonly Row[]): readonly Row[] =>
  rows.length > 0 || value === null || typeof value !== "object" || !isResponse(value)
    ? rows
    : [{ result: resultOf(value.result), rowsAffected: value.rowsAffected }];

export class CommandMiddlewareHost {
  private readonly middlewares: ICommandMiddleware[] = [];

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
