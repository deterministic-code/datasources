import { fill } from "@deterministic-code/generators-common/fill";
import {
  paramsTmpl,
  updateBodyTmpl,
} from "../../resources/procedures-shared.ts";
import type { PackCasing } from "../default-casing.ts";
import { q } from "../sql-dialect.ts";
import {
  pad,
  paramAlignWidth,
  pkFieldOf,
  updatedFieldOf,
  writableNonAuditFields,
} from "./helpers.ts";

export type ProcField = {
  name: string;
  type: string;
};

export type ProcTable = {
  name: string;
  entityName: string;
  fields: ProcField[];
  pkName?: string;
};

export type Param = {
  name: string;
  type: string;
};

export type Variant = {
  byField?: string;
};

type ProcOp =
  | "create"
  | "findOne"
  | "findAll"
  | "findBy"
  | "update"
  | "updateBy"
  | "delete";

type ProcSpec = {
  op: ProcOp;
  name: string;
  byField?: string;
};

export type RenderCtx = {
  entityName: string;
  table: ProcTable;
  tableTok: string;
  pk: ProcField;
  casing: PackCasing;
  columnName: (logical: string) => string;
  routineName: string;
  dialectName: string;
};

export type UpdateSpec = {
  writable: ProcField[];
  name: string;
  params: Param[];
  auditUpdated?: boolean;
};

export type Dialect = {
  dialectName: string;
  paramType(field: ProcField): string;
  generateCreate(ctx: RenderCtx): string;
  generateFindOne(ctx: RenderCtx): string;
  generateFindAll(ctx: RenderCtx): string;
  generateFindBy(ctx: RenderCtx, field: ProcField): string;
  generateUpdate(ctx: RenderCtx, variant: Variant, spec: UpdateSpec): string;
  generateDelete(ctx: RenderCtx): string;
};

/** Ordered CRUD ops + routine names for one entity (CREATE bodies and DROP names). */
export const procedureSpecs = (
  entityName: string,
  byFields: string[],
  casing: PackCasing,
): ProcSpec[] => {
  const name = (stem: string): string => casing.routineName(stem);
  return [
    { op: "create", name: name(`create_${entityName}`) },
    { op: "findOne", name: name(`find_${entityName}`) },
    {
      op: "findAll",
      name: name(`find_${casing.pluralTableName(entityName)}`),
    },
    ...byFields.map((bf) => ({
      op: "findBy" as const,
      name: name(`find_${entityName}_by_${bf}`),
      byField: bf,
    })),
    { op: "update", name: name(`update_${entityName}`) },
    ...byFields.map((bf) => ({
      op: "updateBy" as const,
      name: name(`update_${entityName}_by_${bf}`),
      byField: bf,
    })),
    { op: "delete", name: name(`delete_${entityName}`) },
  ];
};

const requireField = (
  table: ProcTable,
  byField: string | undefined,
  procName: string,
): ProcField => {
  const field = table.fields.find((f) => f.name === byField);
  if (!field) {
    throw new Error(
      `${procName}: field "${byField}" not declared on entity "${table.entityName}"`,
    );
  }
  return field;
};

const hasAuditUpdated = (table: ProcTable): boolean =>
  table.fields.some((f) => f.name === "updated");

const updateSpec = (
  dialect: Dialect,
  table: ProcTable,
  variant: Variant,
  name: string,
  columnName: (logical: string) => string,
): UpdateSpec => {
  const { byField } = variant;
  const key = byField ? requireField(table, byField, name) : pkFieldOf(table);
  const writable = writableNonAuditFields(table).filter((f) => f.name !== byField);
  const col = (field: string): string => columnName(field);
  const audit = hasAuditUpdated(table);
  const updated = audit ? updatedFieldOf(table) : undefined;
  return {
    writable,
    name,
    auditUpdated: audit,
    params: [
      { name: col(key.name), type: dialect.paramType(key) },
      ...writable.map((f) => ({
        name: col(f.name),
        type: dialect.paramType(f),
      })),
      ...(updated
        ? [{ name: col("new_updated"), type: dialect.paramType(updated) }]
        : []),
    ],
  };
};

const renderOp = (dialect: Dialect, spec: ProcSpec, ctx: RenderCtx): string => {
  switch (spec.op) {
    case "create":
      return dialect.generateCreate(ctx);
    case "findOne":
      return dialect.generateFindOne(ctx);
    case "findAll":
      return dialect.generateFindAll(ctx);
    case "findBy":
      return dialect.generateFindBy(
        ctx,
        requireField(ctx.table, spec.byField, spec.name),
      );
    case "update":
    case "updateBy": {
      const variant: Variant =
        spec.op === "updateBy" ? { byField: spec.byField } : {};
      return dialect.generateUpdate(
        ctx,
        variant,
        updateSpec(dialect, ctx.table, variant, spec.name, ctx.columnName),
      );
    }
    case "delete":
      return dialect.generateDelete(ctx);
  }
};

export const generateProceduresFor = (
  dialect: Dialect,
  table: ProcTable,
  byFields: string[],
  casing: PackCasing,
  columnName: (logical: string) => string,
): string[] => {
  const base: Omit<RenderCtx, "routineName"> = {
    entityName: table.entityName,
    table,
    tableTok: q(dialect.dialectName, table.name),
    pk: pkFieldOf(table),
    casing,
    columnName,
    dialectName: dialect.dialectName,
  };
  return procedureSpecs(table.entityName, byFields, casing).map((spec) =>
    renderOp(dialect, spec, { ...base, routineName: spec.name }),
  );
};

/** Aligned `IN`/`@`/bare params; `prefix` is `""` (postgres), `"IN "` (mysql), `"@"` (sqlserver). */
export const renderInParams = (params: Param[], prefix = ""): string => {
  const width = paramAlignWidth(params);
  return fill(paramsTmpl, {
    params: params.map((p, i) => ({
      prefix,
      paddedName: pad(p.name, width),
      type: p.type,
      last: i === params.length - 1,
    })),
  }).trimEnd();
};

export type UpdateProcDialect = {
  colRef(col: string): string;
  setLhs(col: string): string;
  argRef(col: string, procName: string): string;
  wrap(ctx: RenderCtx, spec: UpdateSpec, updateBody: string): string;
};

/** Direct SET (no COALESCE) — PATCH-merge is in the service layer; COALESCE blocked SQL NULL. */
export const makeGenerateUpdate =
  (d: UpdateProcDialect) =>
  (ctx: RenderCtx, variant: Variant, spec: UpdateSpec): string => {
    const col = (field: string): string => ctx.columnName(field);
    const argOf = (field: string) => d.argRef(col(field), spec.name);
    const pk = ctx.pk.name;
    const where = variant.byField
      ? `${d.colRef(col(variant.byField))} = ${argOf(variant.byField)}`
      : `${d.colRef(col(pk))} = ${argOf(pk)}`;
    const sets = [
      ...spec.writable.map((f) => ({
        lhs: d.setLhs(col(f.name)),
        padEq: "    = ",
        rhs: argOf(f.name),
      })),
      ...(spec.auditUpdated === false
        ? []
        : [
            {
              lhs: d.setLhs(col("updated")),
              padEq: " = ",
              rhs: argOf("new_updated"),
            },
          ]),
    ];
    return d.wrap(
      ctx,
      spec,
      fill(updateBodyTmpl, {
        sets: sets.map((s, i) => ({
          ...s,
          first: i === 0,
          last: i === sets.length - 1,
        })),
        where,
      }).trimEnd(),
    );
  };
