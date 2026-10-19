import { BulkUpdateStatementTextRenderer, type Cell } from "@deterministic-code/datasources";
import { columnTypes } from "./columnTypes.ts";
import { isBase64 } from "./converters/BinaryConverter.ts";
import { isIsoDate } from "./converters/DateTimeConverter.ts";

const logical = (value: Cell): string | undefined =>
  value === null
    ? undefined
    : typeof value === "boolean"
      ? "bool"
      : typeof value === "bigint" || (typeof value === "number" && Number.isInteger(value))
        ? "int"
        : typeof value === "number"
          ? "float"
          : value instanceof Date || (typeof value === "string" && isIsoDate(value))
            ? "datetime"
            : value instanceof Uint8Array || (typeof value === "string" && isBase64(value))
              ? "binary"
              : "string";

export class PostgresBulkUpdateStatementTextRenderer extends BulkUpdateStatementTextRenderer {
  protected override cell = (column: string, index: number, value: Cell): string => {
    const type = logical(value);
    const sql = type ? columnTypes[type] : undefined;
    return sql ? `$${column}__${index}::${sql} AS ${column}` : `$${column}__${index} AS ${column}`;
  };
}
