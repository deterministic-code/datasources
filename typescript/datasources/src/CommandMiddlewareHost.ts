import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";
import type { Cell } from "./converters/Cell.ts";

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

  async run<T>(
    query: string,
    params: Parameter[] | undefined,
    fn: (
      query: string,
      params?: Parameter[],
    ) => Promise<{ value: T; rows: readonly Row[] }>,
  ): Promise<T> {
    let q = query.trim();
    let p = params;
    for (const middleware of this.middlewares) {
      const rewritten = await middleware.beforeQuery(q, p);
      if (rewritten) {
        q = rewritten.query;
        p = rewritten.params as Parameter[] | undefined;
      }
    }

    const start = performance.now();
    try {
      const { value, rows } = await fn(q, p);
      const elapsedMs = performance.now() - start;
      for (const middleware of this.middlewares) {
        await middleware.afterQuery(q, p, logged(value, rows), elapsedMs);
      }
      return value;
    } catch (error) {
      const elapsedMs = performance.now() - start;
      for (const middleware of this.middlewares) {
        await middleware.afterQuery(q, p, [], elapsedMs, asError(error));
      }
      throw error;
    }
  }
}
