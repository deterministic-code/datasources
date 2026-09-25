import type { ICommandMiddleware } from "./ICommandMiddleware.ts";
import type { Parameter } from "./Parameter.ts";
import type { Row } from "./Row.ts";
import type { Cell } from "./converters/Cell.ts";

export const formatElapsed = (elapsedMs: number): string =>
  elapsedMs < 1
    ? `${Math.round(elapsedMs * 1000)}μs`
    : `${Math.round(elapsedMs)}ms`;

const cellOf = (value: Cell): string =>
  value === null
    ? "null"
    : value instanceof Date
      ? value.toISOString()
      : value instanceof Uint8Array
        ? Buffer.from(value).toString("base64")
        : typeof value === "bigint"
          ? String(value)
          : typeof value === "boolean"
            ? value ? "True" : "False"
            : typeof value === "string"
              ? value
              : String(value);

const paramsOf = (params?: ReadonlyArray<Parameter>) =>
  `{${(params ?? []).map((param) => `${param.name}: ${cellOf(param.value as Cell)}`).join(", ")}}`;

const rowsOf = (rows: ReadonlyArray<Row>) =>
  rows
    .map((row) => `{${Object.entries(row).sort(([left], [right]) => left < right ? -1 : 1).map(([key, value]) => `${key}: ${cellOf(value)}`).join(", ")}}`)
    .join(", ");

export class ConsoleCommandMiddleware implements ICommandMiddleware {
  constructor(
    private readonly write: (line: string) => void = (line) => {
      console.log(line);
    },
    private readonly now: () => Date = () => new Date(),
    private readonly options: Record<string, string> = {},
  ) {}

  beforeCommand = (text: string, params?: ReadonlyArray<Parameter>): void => {
    this.write(this.line("Start", text));
    if (this.verbose()) this.write(this.line("Request", paramsOf(params)));
  };

  afterCommand = (
    text: string,
    _params: ReadonlyArray<Parameter> | undefined,
    results: ReadonlyArray<Row>,
    elapsedMs: number,
  ): void => {
    const options = { ...this.options, elapsedMs: formatElapsed(elapsedMs) };
    this.write(`${this.line("Finish", text)} ${options.elapsedMs}`);
    if (this.verbose()) this.write(this.line("AfterCommand", rowsOf(results)));
  };

  private verbose = (): boolean =>
    this.options.isVerbose === "true" || this.options.isVerbose === "1";

  private line = (phase: string, text: string): string =>
    `[${this.now().toISOString()}]-[datasource][${phase}]-[${text}]`;
}
