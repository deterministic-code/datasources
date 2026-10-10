import type { Cell, Cells } from "./converters/Cell.ts";
import { DatasourceError } from "./DatasourceError.ts";
import type { Response } from "./Response.ts";
import type { IWhere } from "./statements/IWhere.ts";
import { ExecuteStoredProcedureStatement } from "./statements/ExecuteStoredProcedureStatement.ts";
import type { CRUDRepository } from "./CRUDRepository.ts";
import type { CRUDRepositoryOptions } from "./CRUDRepositoryOptions.ts";
import type { ICommand } from "./ICommand.ts";
import { execute } from "./ICommand.ts";
import type { IBulkWriteStrategy } from "./IBulkWriteStrategy.ts";
import { repositoryParts } from "./CRUDRepositoryBuilder.ts";
import { MetaDataRowTypeConverter } from "./converters/MetaDataRowTypeConverter.ts";
import { afterBulkWrite } from "./BulkWriteEnrichment.ts";

const empty = (): Response => ({ result: null, rowsAffected: 0 });

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
  repo!: CRUDRepository<T>;
  command!: ICommand;
  options!: CRUDRepositoryOptions<T>;

  attach = (command: ICommand, options: CRUDRepositoryOptions<T>): void => {
    this.command = command;
    this.options = options;
  };

  constructor(
    private readonly createName: string,
    private readonly updateName: string,
    private readonly deleteName: string,
  ) {}

  create = (items: T[]): Promise<Response> => this.then(this.call(this.createName, items), items, "create");

  update = (items: T[], _byWhere: IWhere | undefined): Promise<Response> =>
    this.then(this.call(this.updateName, items), items, "update");

  delete = (items: T[], _byWhere: IWhere | undefined): Promise<Response> =>
    this.then(this.call(this.deleteName, items), items, "delete");

  private then = async (write: Promise<Response>, items: T[], mode: "create" | "update" | "delete"): Promise<Response> => {
    const response = await write;
    await afterBulkWrite(this.repo, this.options, items, mode);
    return response;
  };

  private call = (name: string, items: T[]): Promise<Response> => {
    if (items.length === 0) return Promise.resolve(empty());
    const payload = { name: "payload", value: json(items.map(this.rowOf)) };
    return execute(this.command, new ExecuteStoredProcedureStatement(name, [payload]), [payload]);
  };

  private rowOf = (item: T): Cells => {
    const stored = this.reverse(item);
    const statements = repositoryParts(this.command.connection, this.options).statements;
    const row: Cells = {};
    for (const name of statements.readColumns) {
      const match = Object.entries(stored).find(([key]) => this.column(key) === name);
      row[name] = match?.[1] ?? null;
    }
    return row;
  };

  private column = (property: string): string => this.options.propertyMappings?.map(property) ?? property;

  private reverse = (item: T): Cells => {
    const objects = this.options.objectConverter ?? new MetaDataRowTypeConverter<T>();
    const row = objects.reverseConvert(item);
    return this.options.rowConverter ? this.options.rowConverter.reverseConvert(row) : row;
  };
}
