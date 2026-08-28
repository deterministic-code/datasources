import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";

const asError = (error: unknown): Error =>
  error instanceof Error ? error : new Error(String(error));

export class CommandMiddlewareHost {
  private readonly middlewares: ICommandMiddleware[] = [];

  use(middleware: ICommandMiddleware): this {
    this.middlewares.push(middleware);
    return this;
  }

  async run<T>(
    query: string | null | undefined,
    params: Parameter[] | undefined,
    fn: (
      query: string,
      params?: Parameter[],
    ) => Promise<{ value: T; rows: readonly Row[] }>,
  ): Promise<T> {
    let q = (query ?? "").trim();
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
        await middleware.afterQuery(q, p, rows, elapsedMs);
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
