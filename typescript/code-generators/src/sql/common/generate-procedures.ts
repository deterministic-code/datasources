import type { IStatement, ProcedureByGroup, ProcedureSource } from "@deterministic-code/datasources";
import { procedureNameOf } from "@deterministic-code/datasources";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { pkName } from "@deterministic-code/generators-common/spec-types";
import { createCasing, type PackCasing } from "./default-casing.ts";
import { byGroupsFromDatasource } from "./datasource-by-fields.ts";
import { dialectDriver, renderSql, schemaRenderer, type SqlDialect } from "./sql-dialect.ts";
import {
  buildLiveTables,
  datasourceSettings,
  fieldStem,
  hasAuditColumns,
  overlayOf,
  sqlTablesFrom,
  tableIdentity,
  type LiveTable,
} from "./sql-schema.ts";
import { createParamFields, pkFieldOf, updateParamFields } from "./procedures/helpers.ts";
import { writeInclusionOf } from "./write-inclusion.ts";
import { batchSeparatorOf, joinBatches } from "./join-batches.ts";
import { dropRoutineStatementOf } from "../statements.ts";
import { CreateTableStatement } from "@deterministic-code/datasources";

const withNl = (text: string, naming: import("@deterministic-code/datasources").StatementNaming, dialect: SqlDialect): string => {
  const out = schemaRenderer(dialect, naming).applyKeywords(text);
  return out.endsWith("\n") ? out : `${out}\n`;
};

const pad = (n: number): string => String(n).padStart(4, "0");

/** Create / read / update / delete / bulk procedure statements for one live table. */
export const procedureStatementsForTable = (
  dialect: SqlDialect,
  t: LiveTable,
  casing: PackCasing,
  groups: string[][],
): IStatement[] => {
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
  const columnName = (logical: string): string =>
    CreateTableStatement.columnName(fieldStem(t, logical), casing.naming);
  const param = (field: { type: string; name: string }): { name: string; type: string; field: { type: string; name: string } } => ({
    name: columnName(field.name),
    type: field.type,
    field,
  });
  const byGroups: ProcedureByGroup[] = groups.map((logical) => {
    const parameters = logical.map((name) => {
      const field = table.fields.find((f) => f.name === name) ?? pk;
      return param(field);
    });
    const update = [
      ...parameters,
      ...updateParamFields(table).filter((f) => !logical.includes(f.name)).map(param),
    ];
    return {
      logical,
      columns: logical.map(columnName),
      parameters,
      updateParameters: update,
    };
  });
  const source: ProcedureSource = {
    plural: casing.pluralStem(t.name),
    table: t.tableName,
    columns: table.fields.map((f) => columnName(f.name)),
    createParameters: createParamFields(table).map(param),
    writable: updateParamFields(table).map((f) => columnName(f.name)),
    createColumns: createParamFields(table).map(param),
    updateColumns: [
      param(pk),
      ...updateParamFields(table).filter((f) => f.name !== pk.name).map(param),
    ],
    typedColumns: table.fields.map(param),
    byGroups,
    key: columnName(pk.name),
    keyType: pk.type,
    auditUpdated: table.fields.some((f) => f.name === "updated" && f.includeInUpdate)
      ? columnName("updated")
      : undefined,
    generatedKey: pk.type === "uuid",
    routineName: casing.routineName,
  };
  return dialectDriver(dialect).statementRenderer!.procedures(source);
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
  const tables = buildLiveTables(types, casing.naming).filter(hasAuditColumns);
  if (tables.length === 0) return [];

  const separator = batchSeparatorOf(ctx.settings);
  const byGroups = byGroupsFromDatasource(types);
  const bodies: string[] = [];
  const drops: string[] = [];
  for (const t of tables) {
    const statements = procedureStatementsForTable(
      dialect,
      t,
      casing,
      byGroups.get(t.name) ?? [],
    );
    const created = statements.map((statement) => renderSql(dialect, statement)).filter((sql) => sql !== "");
    for (const statement of statements) {
      const drop = renderSql(dialect, dropRoutineStatementOf(procedureNameOf(statement)));
      if (drop !== "") drops.push(drop);
    }
    if (created.length > 0) bodies.push(`-- ${t.name}\n\n${joinBatches(created, separator)}`);
  }
  if (bodies.length === 0) return [];

  return [
    content(
      `${dialect}/migrations/${pad(order)}_stored_procedures_up.sql`,
      withNl(`-- Stored procedures\n${joinBatches(bodies, separator)}`, casing.naming, dialect),
    ),
    content(
      `${dialect}/migrations/${pad(order)}_stored_procedures_down.sql`,
      withNl(`-- DROP stored procedures\n${joinBatches(drops, separator)}`, casing.naming, dialect),
    ),
  ];
};
