import type { IStatement } from "@deterministic-code/datasources";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { pkName } from "@deterministic-code/generators-common/spec-types";
import { createCasing, type PackCasing } from "./default-casing.ts";
import { byGroupsFromDatasource } from "./datasource-by-fields.ts";
import { SqlMapping } from "./sql-mapping.ts";
import { dialectDriver, renderSql, type SqlDialect } from "./sql-dialect.ts";
import {
  buildLiveTables,
  datasourceSettings,
  hasAuditColumns,
  overlayOf,
  sqlTablesFrom,
  tableIdentity,
  type LiveTable,
} from "./sql-schema.ts";
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

const withNl = (text: string, casing: PackCasing, dialect: SqlDialect): string => {
  const out = casing.applyKeywords(text, dialect);
  return out.endsWith("\n") ? out : `${out}\n`;
};

const pad = (n: number): string => String(n).padStart(4, "0");


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
  const param = (field: { type: string; name: string }): { name: string; type: string; field: { type: string; name: string } } => ({
    name: columnName(field.name),
    type: field.type,
    field,
  });
  const columns = table.fields.map((f) => columnName(f.name));
  const writable = updateParamFields(table).map((f) => columnName(f.name));
  const createdParams = createParamFields(table).map(param);
  const typedColumns = table.fields.map(param);
  const createColumns = createParamFields(table).map(param);
  const updateColumns = [
    param(pk),
    ...updateParamFields(table)
      .filter((f) => f.name !== pk.name)
      .map(param),
  ];
  const payload = { name: "payload", type: "payload" };
  const key = columnName(pk.name);
  const keyType = pk.type;
  const audit = table.fields.some((f) => f.name === "updated" && f.includeInUpdate) ? columnName("updated") : undefined;
  const generatedKey = pk.type === "uuid";
  const tableName = t.tableName;
  const statements: IStatement[] = [];
  const names: string[] = [];
  const specs = (
    dialectDriver(dialect).constructor as {
      procedureSpecs: (
        entity: string,
        groups: string[][],
        casing: { routineName: (stem: string) => string; pluralStem: (entity: string) => string },
      ) => { op: string; name: string; byProperties: string[] }[];
    }
  ).procedureSpecs;
  for (const proc of specs(t.name, groups, casing)) {
    names.push(proc.name);
    const by = proc.byProperties.map(columnName);
    const byParams = proc.byProperties.map((logical) => {
      const field = table.fields.find((f) => f.name === logical) ?? pk;
      return param(field);
    });
    const updateParams = [
      ...byParams,
      ...updateParamFields(table)
        .filter((f) => !proc.byProperties.includes(f.name))
        .map(param),
    ];
    statements.push(
      ({
        create: () =>
          createCreateStoredProcedureOf(proc.name, tableName, columns, createdParams, key, keyType, writable, generatedKey, audit),
        read: () =>
          createReadStoredProcedureOf(proc.name, tableName, columns, byParams, key, keyType, writable, by, generatedKey, audit),
        update: () =>
          createUpdateStoredProcedureOf(proc.name, tableName, columns, updateParams, key, keyType, writable, by, generatedKey, audit),
        delete: () =>
          createDeleteStoredProcedureOf(proc.name, tableName, columns, byParams, key, keyType, writable, by, generatedKey, audit),
        bulkCreate: () =>
          createBulkCreateStoredProcedureOf(proc.name, tableName, createColumns, payload, key, keyType),
        bulkUpdate: () =>
          createBulkUpdateStoredProcedureOf(proc.name, tableName, updateColumns, payload, key, keyType),
        bulkDelete: () =>
          createBulkDeleteStoredProcedureOf(proc.name, tableName, typedColumns, payload, key, keyType),
      }[proc.op as "create"])(),
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
      withNl(`-- Stored procedures\n${joinBatches(bodies, separator)}`, casing, dialect),
    ),
    content(
      `${dialect}/migrations/${pad(order)}_stored_procedures_down.sql`,
      withNl(`-- DROP stored procedures\n${joinBatches(drops, separator)}`, casing, dialect),
    ),
  ];
};
