import { createHash } from "node:crypto";
import type { SeedRow, SeedValue } from "@deterministic-code/deterministic-generators-common/parser";
import { isPkField } from "@deterministic-code/generators-common/spec-types";
import { SqlSeedStatement, type SpecField } from "@deterministic-code/datasources";
import { renderSql, type SqlDialect } from "./sql-dialect.ts";
import { converterFieldOf } from "./converter-field.ts";
import type { SqlMapping } from "./sql-mapping.ts";
import {
  isGeneratedIdentity,
  overlayOf,
  tableIdentity,
  usesGeneratedIdColumn,
  type LiveTable,
} from "./sql-schema.ts";

const SEED_UUID_NAMESPACE = "9b3a8e6c-2f1d-4a5b-8c9d-1e2f3a4b5c6d";

const pkType = (table: LiveTable): string =>
  table.fields.find((f) => isPkField(f, table, overlayOf(table)))?.type ?? "integer";

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

const idValue = (table: LiveTable, id: number): unknown => {
  const t = pkType(table);
  if (t === "uuid") return seedUuid(table.name, id);
  if (t === "string") return String(id);
  return id;
};

const colsForRow = (table: LiveTable, row: Record<string, SeedValue>): string[] => {
  const names = new Set(table.fields.map((f) => f.name));
  const generated = isGeneratedIdentity(table) ? tableIdentity(table) : [];
  return [
    ...generated.filter((k) => names.has(k)),
    ...(names.has("uuid") && !generated.includes("uuid") ? ["uuid"] : []),
    ...Object.keys(row).filter((k) => !generated.includes(k) && k !== "uuid"),
  ];
};

const columnOf = (
  table: LiveTable,
  mapping: SqlMapping,
  logical: string,
  seed: SeedRow,
): { column: string; field: SpecField; value: unknown } => {
  const field = table.fields.find((f) => f.name === logical);
  const spec = field ? converterFieldOf(field) : { type: "string", name: logical };
  const generated = isGeneratedIdentity(table) ? tableIdentity(table) : [];
  const value = generated.includes(logical)
    ? idValue(table, seed.id)
    : logical === "uuid"
      ? seedUuid(table.name, seed.id)
      : seed.row[logical];
  return { column: mapping.columnName(table, logical), field: spec, value };
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
    const pk = table.fields.find((field) => field.name === tableIdentity(table)[0]);
    const identity =
      pk != null && usesGeneratedIdColumn(table, pk) && pk.defaultValue == null;
    lines.push(`-- Seeds: ${table.tableName}`);
    for (const seed of rows) {
      const cols = colsForRow(table, seed.row);
      const columns = cols.map((c) => columnOf(table, mapping, c, seed));
      lines.push(
        renderSql(
          dialect,
          new SqlSeedStatement(mapping.tableName(table), columns, "insert", identity, pk ? mapping.columnName(table, pk.name) : undefined),
        ),
      );
    }
  }
  return lines;
};

export const addedSeeds = (
  previous: Map<string, SeedRow[]>,
  current: Map<string, SeedRow[]>,
): Map<string, SeedRow[]> => {
  const added = new Map<string, SeedRow[]>();
  for (const [table, rows] of current) {
    const ids = new Set((previous.get(table) ?? []).map((row) => row.id));
    const next = rows.filter((row) => !ids.has(row.id));
    if (next.length > 0) added.set(table, next);
  }
  return added;
};

export const seedDeletes = (
  dialect: SqlDialect,
  tables: LiveTable[],
  seeds: Map<string, SeedRow[]>,
  mapping: SqlMapping,
): string[] => {
  const lines: string[] = [];
  for (const table of tables) {
    const rows = seeds.get(table.name) ?? [];
    if (rows.length === 0) continue;
    const id = tableIdentity(table)[0] ?? "id";
    const field = table.fields.find((f) => f.name === id);
    lines.push(`-- Seeds: ${table.tableName}`);
    for (const seed of rows)
      lines.push(
        renderSql(
          dialect,
          new SqlSeedStatement(
            mapping.tableName(table),
            [
              {
                column: mapping.columnName(table, id),
                field: field ? converterFieldOf(field) : { type: pkType(table) },
                value: idValue(table, seed.id),
              },
            ],
            "delete",
          ),
        ),
      );
  }
  return lines;
};
