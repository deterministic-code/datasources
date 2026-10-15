import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { pkName } from "@deterministic-code/generators-common/spec-types";
import { createCasing, type PackCasing } from "./default-casing.ts";
import { byFieldsFromDatasource } from "./datasource-by-fields.ts";
import { SqlMapping } from "./sql-mapping.ts";
import { mapColumnType, q, renderSql, type SqlDialect } from "./sql-dialect.ts";
import {
  buildLiveTables,
  datasourceSettings,
  hasAuditColumns,
  overlayOf,
  sqlTablesFrom,
} from "./sql-schema.ts";
import { procedureSpecs } from "./procedures/driver.ts";
import {
  createParamFields,
  pkFieldOf,
  writableNonAuditFields,
} from "./procedures/helpers.ts";
import { createRoutineStatementOf, dropRoutineStatementOf } from "../statements.ts";

const withNl = (text: string, casing: PackCasing): string => {
  const out = casing.applyKeywords(text);
  return out.endsWith("\n") ? out : `${out}\n`;
};

/** Stored-procedure migration — empty when the driver does not render routines. */
export const generateProceduresForDialect = async (
  dialect: SqlDialect,
  ctx: GenerateContext,
): Promise<GenerateEntry[]> => {
  if (!datasourceSettings(ctx.settings).useStoredProcedures) return [];

  const spec = await DeterministicParser(ctx.reader).parse(ctx.settings);
  const types = sqlTablesFrom(spec);
  const casing = createCasing(ctx.settings);
  const mapping = new SqlMapping(casing, types);
  const tables = buildLiveTables(types, mapping).filter(hasAuditColumns);
  if (tables.length === 0) return [];

  const byFields = byFieldsFromDatasource(types);
  const bodies: string[] = [];
  const drops: string[] = [];
  for (const t of tables) {
    const fields = byFields.get(t.name) ?? [];
    const table = {
      name: t.tableName,
      entityName: t.name,
      fields: t.fields.map((f) => ({ name: f.name, type: f.type })),
      pkName: pkName(t, overlayOf(t)),
    };
    const pk = pkFieldOf(table);
    const columnName = (logical: string): string => mapping.columnName(t, logical);
    const columns = table.fields.map((f) => columnName(f.name));
    const writable = writableNonAuditFields(table).map((f) => columnName(f.name));
    const paramType = (field: { type: string; name: string }): string =>
      mapColumnType(dialect, field);
    const created: string[] = [];
    for (const proc of procedureSpecs(t.name, fields, casing)) {
      const by = proc.byField;
      const keyField = by
        ? table.fields.find((f) => f.name === by) ?? pk
        : pk;
      const params = [
        { name: columnName(keyField.name), type: paramType(keyField) },
        ...writableNonAuditFields(table)
          .filter((f) => f.name !== by)
          .map((f) => ({ name: columnName(f.name), type: paramType(f) })),
        ...(table.fields.some((f) => f.name === "updated")
          ? [{ name: columnName("new_updated"), type: paramType(table.fields.find((f) => f.name === "updated")!) }]
          : []),
      ];
      const createParams = createParamFields(table).map((f) => ({
        name: columnName(f.name),
        type: paramType(f),
      }));
      const sql = renderSql(
        dialect,
        createRoutineStatementOf(
          proc.name,
          proc.op === "updateBy" ? "update" : proc.op,
          q(dialect, t.tableName),
          columns,
          proc.op === "create"
            ? createParams
            : proc.op.startsWith("update")
              ? params
              : proc.op === "findBy"
                ? [{ name: columnName(keyField.name), type: paramType(keyField) }]
                : [],
          columnName(pk.name),
          paramType(pk),
          writable,
          by === undefined ? undefined : columnName(by),
          pk.type === "uuid",
          table.fields.some((f) => f.name === "updated") ? columnName("updated") : undefined,
        ),
      );
      if (sql !== "") created.push(sql);
      const drop = renderSql(dialect, dropRoutineStatementOf(proc.name));
      if (drop !== "") drops.push(drop);
    }
    if (created.length > 0) bodies.push(`-- ${t.name}\n\n${created.join("\n\n")}`);
  }
  if (bodies.length === 0) return [];

  return [
    content(
      `${dialect}/migrations/0002_stored_procedures_up.sql`,
      withNl(`-- Stored procedures\n${bodies.join("\n\n")}`, casing),
    ),
    content(
      `${dialect}/migrations/0002_stored_procedures_down.sql`,
      withNl(`-- DROP stored procedures\n${drops.join("\n")}`, casing),
    ),
  ];
};
