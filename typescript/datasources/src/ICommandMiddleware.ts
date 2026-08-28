import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";

export interface ICommandMiddleware {
  beforeQuery(
    query: string,
    params?: ReadonlyArray<Parameter>,
  ):
    | void
    | { query: string; params?: ReadonlyArray<Parameter> }
    | Promise<void | { query: string; params?: ReadonlyArray<Parameter> }>;

  afterQuery(
    query: string,
    params: ReadonlyArray<Parameter> | undefined,
    results: ReadonlyArray<Row>,
    elapsedMs: number,
    error?: Error,
  ): void | Promise<void>;
}
