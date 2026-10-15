import { createHash } from "node:crypto";
import type {
  SeedRow,
  SeedValue,
  TypeField,
} from "@deterministic-code/deterministic-generators-common/parser";
import { isPkField } from "@deterministic-code/generators-common/spec-types";
import { sqlStringLiteral } from "@datasources/core";
import { identityInsertStatementOf, seedInsertStatementOf } from "../statements.ts";
import { renderSql, sqlDefault, type SqlDialect } from "./sql-dialect.ts";
import { dialectConverter } from "./sql-dialect.ts";
import type { SqlMapping } from "./sql-mapping.ts";
import {
  isGeneratedIdentity,
  overlayOf,
  tableIdentity,
  type LiveTable,
} from "./sql-schema.ts";

const SEED_UUID_NAMESPACE = "9b3a8e6c-2f1d-4a5b-8c9d-1e2f3a4b5c6d";

const pkType = (table: LiveTable): string =>
  table.fields.find((f) => isPkField(f, table, overlayOf(table)))?.type ??
  "integer";

const seedUuid = (tableName: string, id: number): string => {
  const bytes = Buffer.from(
    createHash("sha1")
      .update(
        Buffer.concat([
          Buffer.from(SEED_UUID_NAMESPACE.replace(/-/g, ""), "hex"),
          Buffer.from(`${tableName}:${id}`, "utf8"),
        ]),
      )
      .digest()
      .subarray(0, 16),
  );
  bytes[6] = (bytes[6] & 0x0f) | 0x50;
  bytes[8] = (bytes[8] & 0x3f) | 0x80;
  const hex = bytes.toString("hex");
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20, 32)}`;
};

const renderValue = (
  dialect: SqlDialect,
  field: TypeField,
  value: SeedValue,
): string => {
  if (field.type === "boolean") {
    return (
      dialectConverter(dialect).defaultValueConverter.convert(
        "boolean",
        value ? "true" : "false",
      ) ?? "0"
    );
  }
  if (field.type === "number" || field.type === "reference") {
    return String(value);
  }
  return sqlStringLiteral(value);
};

const idValue = (table: LiveTable, id: number): string => {
  const t = pkType(table);
  if (t === "uuid") return sqlStringLiteral(seedUuid(table.name, id));
  if (t === "string") return sqlStringLiteral(String(id));
  return String(id);
};

const colValue = (
  dialect: SqlDialect,
  field: TypeField | undefined,
  value: SeedValue | undefined,
): string => {
  if (!field) return "NULL";
  if (value === null || value === undefined) {
    return sqlDefault(dialect, field) ?? "NULL";
  }
  return renderValue(dialect, field, value);
};

const colsForRow = (
  table: LiveTable,
  row: Record<string, SeedValue>,
): string[] => {
  const names = new Set(table.fields.map((f) => f.name));
  const generated = isGeneratedIdentity(table) ? tableIdentity(table) : [];
  return [
    ...generated.filter((k) => names.has(k)),
    ...(names.has("uuid") && !generated.includes("uuid") ? ["uuid"] : []),
    ...Object.keys(row).filter((k) => !generated.includes(k) && k !== "uuid"),
  ];
};

/** Per-table seed SQL blocks for the initial migration (`-- Seeds: <table>`). */
export const seedSections = (
  dialect: SqlDialect,
  tables: LiveTable[],
  seeds: Map<string, SeedRow[]>,
  mapping: SqlMapping,
): string[] => {
  const lines: string[] = [];
  for (const table of tables) {
    const rows = seeds.get(table.name) ?? [];
    if (rows.length === 0) continue;
    const generated = isGeneratedIdentity(table) ? tableIdentity(table) : [];
    const t = pkType(table);
    const identity = isGeneratedIdentity(table) && t !== "uuid" && t !== "string";
    const inserts = rows.map((seed) => {
      const cols = colsForRow(table, seed.row);
      const values = cols.map((c) => {
        if (generated.includes(c)) return idValue(table, seed.id);
        if (c === "uuid") return sqlStringLiteral(seedUuid(table.name, seed.id));
        return colValue(
          dialect,
          table.fields.find((f) => f.name === c),
          seed.row[c],
        );
      });
      return seedInsertStatementOf(table, mapping, cols, values, dialect);
    });
    lines.push(
      `-- Seeds: ${table.tableName}`,
      renderSql(dialect, identityInsertStatementOf(dialect, table, mapping, inserts, identity)),
      "",
    );
  }
  return lines;
};
