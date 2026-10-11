import { BulkUpdateStatementTextRenderer, type Cell } from "@datasources/core";
import { columnTypes } from "./columnTypes.ts";

const logical = (value: Cell): string | undefined =>
  value === null
    ? undefined
    : typeof value === "boolean"
      ? "bool"
      : typeof value === "bigint" || (typeof value === "number" && Number.isInteger(value))
        ? "int"
        : typeof value === "number"
          ? "float"
          : value instanceof Date
            ? "datetime"
            : value instanceof Uint8Array
              ? "binary"
              : "string";

export class PostgresBulkUpdateStatementTextRenderer extends BulkUpdateStatementTextRenderer {
  protected override cell = (column: string, index: number, value: Cell): string => {
    const type = logical(value);
    const sql = type ? columnTypes[type] : undefined;
    return sql ? `$${column}__${index}::${sql} AS ${column}` : `$${column}__${index} AS ${column}`;
  };
}
