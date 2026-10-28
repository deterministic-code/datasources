import type { IStatement } from "@deterministic-code/datasources";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { pkName } from "@deterministic-code/generators-common/spec-types";
import { createCasing, type PackCasing } from "./default-casing.ts";
import { byGroupsFromDatasource } from "./datasource-by-fields.ts";
import { SqlMapping } from "./sql-mapping.ts";
import { mapColumnType, q, renderSql, type SqlDialect } from "./sql-dialect.ts";
import {
  buildLiveTables,
  datasourceSettings,
  hasAuditColumns,
  overlayOf,
  sqlTablesFrom,
  tableIdentity,
  type LiveTable,
} from "./sql-schema.ts";
import { procedureSpecs } from "./procedures/driver.ts";
import { createParamFields, pkFieldOf, updateParamFields } from "./procedures/helpers.ts";
import { writeInclusionOf } from "./write-inclusion.ts";
import { batchSeparatorOf, joinBatches } from "./join-batches.ts";
import {
  createBulkCreateStoredProcedureOf,
  createBulkDeleteStoredProcedureOf,
  createBulkUpdateStoredProcedureOf,
  createCreateStoredProcedureOf,
  createDeleteStoredProcedureOf,
  createReadStoredProcedureOf,
  createUpdateStoredProcedureOf,
  dropRoutineStatementOf,
} from "../statements.ts";

const withNl = (text: string, casing: PackCasing): string => {
  const out = casing.applyKeywords(text);
  return out.endsWith("\n") ? out : `${out}\n`;
};

const pad = (n: number): string => String(n).padStart(4, "0");

const payloadType = (dialect: SqlDialect): string =>
  ({
    mysql: "JSON",
    postgres: "jsonb",
    sqlserver: "NVARCHAR(MAX)",
    oracle: "CLOB",
  }[dialect] ?? "JSON");

/** Create / read / update / delete / bulk procedure statements for one live table. */
export const procedureStatementsForTable = (
  dialect: SqlDialect,
  t: LiveTable,
  mapping: SqlMapping,
  casing: PackCasing,
  groups: string[][],
): { statements: IStatement[]; names: string[] } => {
  const overlay = overlayOf(t);
  const ids = tableIdentity(t);
  const table = {
    name: t.tableName,
    entityName: t.name,
    fields: t.fields.map((f) => {
      const flags = writeInclusionOf(
        f,
        ids,
        overlay.fields.find((item) => item.name === f.name),
      );
      return { name: f.name, type: f.type, ...flags };
    }),
    pkName: pkName(t, overlay),
  };
  const pk = pkFieldOf(table);
  const columnName = (logical: string): string => mapping.columnName(t, logical);
  const columns = table.fields.map((f) => columnName(f.name));
  const writable = updateParamFields(table).map((f) => columnName(f.name));
  const paramType = (field: { type: string; name: string }): string =>
    mapColumnType(dialect, field);
  const createdParams = createParamFields(table).map((f) => ({
    name: columnName(f.name),
    type: paramType(f),
  }));
  const typedColumns = table.fields.map((f) => ({ name: columnName(f.name), type: paramType(f) }));
  const createColumns = createParamFields(table).map((f) => ({ name: columnName(f.name), type: paramType(f) }));
  const updateColumns = [
    { name: columnName(pk.name), type: paramType(pk) },
    ...updateParamFields(table)
      .filter((f) => f.name !== pk.name)
      .map((f) => ({ name: columnName(f.name), type: paramType(f) })),
  ];
  const payload = { name: "payload", type: payloadType(dialect) };
  const key = columnName(pk.name);
  const keyType = paramType(pk);
  const audit = table.fields.some((f) => f.name === "updated" && f.includeInUpdate) ? columnName("updated") : undefined;
  const generatedKey = pk.type === "uuid";
  const quoted = q(dialect, t.tableName);
  const statements: IStatement[] = [];
  const names: string[] = [];
  for (const proc of procedureSpecs(t.name, groups, casing)) {
    names.push(proc.name);
    const by = proc.byProperties.map(columnName);
    const byParams = proc.byProperties.map((logical) => {
      const field = table.fields.find((f) => f.name === logical) ?? pk;
      return { name: columnName(field.name), type: paramType(field) };
    });
    const updateParams = [
      ...byParams,
      ...updateParamFields(table)
        .filter((f) => !proc.byProperties.includes(f.name))
        .map((f) => ({ name: columnName(f.name), type: paramType(f) })),
    ];
    statements.push(
      ({
        create: () =>
          createCreateStoredProcedureOf(proc.name, quoted, columns, createdParams, key, keyType, writable, generatedKey, audit),
        read: () =>
          createReadStoredProcedureOf(proc.name, quoted, columns, byParams, key, keyType, writable, by, generatedKey, audit),
        update: () =>
          createUpdateStoredProcedureOf(proc.name, quoted, columns, updateParams, key, keyType, writable, by, generatedKey, audit),
        delete: () =>
          createDeleteStoredProcedureOf(proc.name, quoted, columns, byParams, key, keyType, writable, by, generatedKey, audit),
        bulkCreate: () =>
          createBulkCreateStoredProcedureOf(proc.name, quoted, createColumns, payload, key, keyType),
        bulkUpdate: () =>
          createBulkUpdateStoredProcedureOf(proc.name, quoted, updateColumns, payload, key, keyType),
        bulkDelete: () =>
          createBulkDeleteStoredProcedureOf(proc.name, quoted, typedColumns, payload, key, keyType),
      }[proc.op])(),
    );
  }
  return { statements, names };
};

/** Stored-procedure migration — empty when the driver does not render routines. */
export const generateProceduresForDialect = async (
  dialect: SqlDialect,
  ctx: GenerateContext,
  order = 2,
): Promise<GenerateEntry[]> => {
  if (!datasourceSettings(ctx.settings).useStoredProcedures) return [];

  const spec = await DeterministicParser(ctx.reader).parse(ctx.settings);
  const types = sqlTablesFrom(spec);
  const casing = createCasing(ctx.settings);
  const mapping = new SqlMapping(casing, types);
  const tables = buildLiveTables(types, mapping).filter(hasAuditColumns);
  if (tables.length === 0) return [];

  const separator = batchSeparatorOf(ctx.settings);
  const byGroups = byGroupsFromDatasource(types);
  const bodies: string[] = [];
  const drops: string[] = [];
  for (const t of tables) {
    const { statements, names } = procedureStatementsForTable(
      dialect,
      t,
      mapping,
      casing,
      byGroups.get(t.name) ?? [],
    );
    const created = statements.map((statement) => renderSql(dialect, statement)).filter((sql) => sql !== "");
    for (const name of names) {
      const drop = renderSql(dialect, dropRoutineStatementOf(name));
      if (drop !== "") drops.push(drop);
    }
    if (created.length > 0) bodies.push(`-- ${t.name}\n\n${joinBatches(created, separator)}`);
  }
  if (bodies.length === 0) return [];

  return [
    content(
      `${dialect}/migrations/${pad(order)}_stored_procedures_up.sql`,
      withNl(`-- Stored procedures\n${joinBatches(bodies, separator)}`, casing),
    ),
    content(
      `${dialect}/migrations/${pad(order)}_stored_procedures_down.sql`,
      withNl(`-- DROP stored procedures\n${joinBatches(drops, separator)}`, casing),
    ),
  ];
};
