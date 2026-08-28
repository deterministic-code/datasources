import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";

export const formatElapsed = (elapsedMs: number): string =>
  elapsedMs < 1
    ? `${Math.round(elapsedMs * 1000)}μs`
    : `${Math.round(elapsedMs)}ms`;

export class ConsoleCommandMiddleware implements ICommandMiddleware {
  constructor(
    private readonly write: (line: string) => void = (line) => {
      console.log(line);
    },
    private readonly now: () => Date = () => new Date(),
  ) {}

  beforeQuery(query: string): void {
    this.write(this.line("Start", query));
  }

  afterQuery(
    query: string,
    _params: ReadonlyArray<Parameter> | undefined,
    _results: ReadonlyArray<Row>,
    elapsedMs: number,
  ): void {
    this.write(`${this.line("Finish", query)} ${formatElapsed(elapsedMs)}`);
  }

  private line(phase: string, query: string): string {
    return `[${this.now().toISOString()}]-[datasource][${phase}]-[${query}]`;
  }
}
