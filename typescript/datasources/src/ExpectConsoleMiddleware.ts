import { ConsoleCommandMiddleware } from "./ConsoleCommandMiddleware.ts";
import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";

export const consoleSignature = (line: string): string => {
  const duration = line.lastIndexOf("] ");
  return duration >= 0 && (line.endsWith("ms") || line.endsWith("μs"))
    ? line.slice(0, duration + 1)
    : line;
};

export class ExpectConsoleMiddleware implements ICommandMiddleware {
  private readonly inner: ConsoleCommandMiddleware;
  private index = 0;

  constructor(
    private readonly expected: readonly string[],
    now: () => Date,
  ) {
    this.inner = new ConsoleCommandMiddleware((line) => this.check(line), now, { isVerbose: "true" });
  }

  beforeCommand = (text: string, params?: ReadonlyArray<Parameter>) =>
    this.inner.beforeCommand(text, params);

  afterCommand = (
    text: string,
    params: ReadonlyArray<Parameter> | undefined,
    results: ReadonlyArray<Row>,
    elapsedMs: number,
    error?: Error,
  ) => this.inner.afterCommand(text, params, results, elapsedMs, error);

  complete = (): void => {
    if (this.index !== this.expected.length)
      throw new Error(`console mismatch at ${this.index}\nexpected: ${this.expected[this.index]}\nactual:   <missing>`);
  };

  private check = (line: string): void => {
    const actual = consoleSignature(line);
    if (this.index >= this.expected.length || actual !== this.expected[this.index])
      throw new Error(`console mismatch at ${this.index}\nexpected: ${this.index < this.expected.length ? this.expected[this.index] : "<missing>"}\nactual:   ${actual}`);
    this.index++;
  };
}
