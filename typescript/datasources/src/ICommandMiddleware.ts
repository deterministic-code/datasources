import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";

export interface ICommandMiddleware {
  beforeCommand(
    text: string,
    params?: ReadonlyArray<Parameter>,
  ):
    | void
    | { text: string; params?: ReadonlyArray<Parameter> }
    | Promise<void | { text: string; params?: ReadonlyArray<Parameter> }>;

  afterCommand(
    text: string,
    params: ReadonlyArray<Parameter> | undefined,
    results: ReadonlyArray<Row>,
    elapsedMs: number,
    error?: Error,
  ): void | Promise<void>;
}
