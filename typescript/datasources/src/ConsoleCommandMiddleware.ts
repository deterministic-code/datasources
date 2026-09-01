import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";
import type { Cell } from "./converters/Cell.ts";

export const formatElapsed = (elapsedMs: number): string =>
  elapsedMs < 1
    ? `${Math.round(elapsedMs * 1000)}μs`
    : `${Math.round(elapsedMs)}ms`;

const cellOf = (value: Cell): string =>
  value instanceof Date
    ? value.toISOString()
    : value instanceof Uint8Array
      ? Buffer.from(value).toString("base64")
      : typeof value === "bigint"
        ? String(value)
        : JSON.stringify(value);

const paramsOf = (params?: ReadonlyArray<Parameter>) =>
  `{${(params ?? []).map((param) => `${param.name}: ${cellOf(param.value as Cell)}`).join(", ")}}`;

const rowsOf = (rows: ReadonlyArray<Row>) =>
  rows.map((row) => `{${Object.entries(row).map(([key, value]) => `${key}: ${cellOf(value)}`).join(", ")}}`).join(", ");

export class ConsoleCommandMiddleware implements ICommandMiddleware {
  constructor(
    private readonly write: (line: string) => void = (line) => {
      console.log(line);
    },
    private readonly now: () => Date = () => new Date(),
    private readonly options: Record<string, string> = {},
  ) {}

  beforeQuery = (query: string, params?: ReadonlyArray<Parameter>): void => {
    this.write(this.line("Start", query));
    if (this.verbose()) this.write(this.line("Request", paramsOf(params)));
  };

  afterQuery = (
    query: string,
    _params: ReadonlyArray<Parameter> | undefined,
    results: ReadonlyArray<Row>,
    elapsedMs: number,
  ): void => {
    const options = { ...this.options, elapsedMs: formatElapsed(elapsedMs) };
    this.write(`${this.line("Finish", query)} ${options.elapsedMs}`);
    if (this.verbose()) this.write(this.line("Response", rowsOf(results)));
  };

  private verbose = (): boolean =>
    this.options.isVerbose === "true" || this.options.isVerbose === "1";

  private line = (phase: string, query: string): string =>
    `[${this.now().toISOString()}]-[datasource][${phase}]-[${query}]`;
}
