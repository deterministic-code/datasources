import type { Cell, Cells } from "./converters/Cell.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { BulkCRUDRepository } from "./BulkCRUDRepository.ts";
import { execute } from "./ICommand.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";

const empty = (): Response => ({ result: null, rowsAffected: 0 });

const call = <T extends object>(repo: BulkCRUDRepository<T>, name: string, items: T[]): Promise<Response> => {
  if (items.length === 0) return Promise.resolve(empty());
  const payload = { name: "payload", value: json(repo.rows(items)) };
  return execute(repo.command(), new ExecuteStoredProcedureStatement(name, [payload]), [payload]);
};

const json = (rows: Cells[]): string => `[${rows.map(object).join(",")}]`;

const object = (row: Cells): string =>
  `{${Object.entries(row)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([key, value]) => `${quote(key)}:${scalar(value)}`)
    .join(",")}}`;

const scalar = (value: Cell): string => {
  if (value === null || value === undefined) return "null";
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") return Number.isFinite(value) ? JSON.stringify(value) : fail();
  if (typeof value === "bigint") return value.toString();
  if (typeof value === "string") return quote(value);
  if (value instanceof Date) return quote(roundTrip(value));
  if (value instanceof Uint8Array) return quote(base64(value));
  return fail();
};

const fail = (): string => {
  throw new DatasourceError("Unsupported bulk payload value");
};

const roundTrip = (date: Date): string =>
  date.toISOString().replace(/\.(\d{3})Z$/, (_, fraction: string) => `.${fraction}0000Z`);

const base64 = (bytes: Uint8Array): string =>
  btoa(Array.from(bytes, (byte) => String.fromCharCode(byte)).join(""));

const quote = (text: string): string => {
  let result = "\"";
  for (const ch of text) {
    result += ({
      "\\": "\\\\",
      "\"": "\\\"",
      "\b": "\\b",
      "\f": "\\f",
      "\n": "\\n",
      "\r": "\\r",
      "\t": "\\t",
    }[ch] ?? (ch < " " ? `\\u${ch.charCodeAt(0).toString(16).padStart(4, "0")}` : ch));
  }
  return `${result}"`;
};

export class StoredProcedureBulkWriteStrategy<T extends object> implements IBulkWriteStrategy<T> {
  constructor(
    private readonly createName: string,
    private readonly updateName: string,
    private readonly deleteName: string,
  ) {}

  create = (items: T[], repo: BulkCRUDRepository<T>): Promise<Response> => call(repo, this.createName, items);

  update = (items: T[], _byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response> =>
    call(repo, this.updateName, items);

  delete = (items: T[], _byWhere: IWhere | undefined, repo: BulkCRUDRepository<T>): Promise<Response> =>
    call(repo, this.deleteName, items);
}
