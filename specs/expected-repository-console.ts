import {
  Literal,
  Operator,
  SelectStatement,
  InsertStatement,
  UpdateStatement,
  DeleteStatement,
  Where,
  type Field,
  type IExpression,
  type Scalar,
} from "@datasources/core";
import { expectedConsoleSignatures } from "./expected-console-output.ts";

export type CanonicalType =
  | "int"
  | "uuid"
  | "string"
  | "bool"
  | "decimal"
  | "float"
  | "datetime"
  | "binary";

export type RepositoryMappings = Record<string, Record<string, string>>;

export type CRUDMapperSpec = {
  entity?: string;
  querySuffix?: string;
  writeSuffix?: string;
};

export type RepositoryTypeSpec = {
  name: string;
  table: string;
  ids: string[];
  fields: Record<string, CanonicalType>;
  converters?: Record<string, CanonicalType>;
  mappings?: RepositoryMappings;
  crudMapper?: CRUDMapperSpec;
};

export type RepositoryRow = { [key: string]: Scalar | RepositoryRow };

export type AffectedExpect = { rowsAffected: number };

export type RepositoryStep = {
  CREATE?: { row: RepositoryRow };
  READ?: { id: Scalar; expect?: RepositoryRow | null };
  QUERY?: { where?: RepositoryRow; expect?: RepositoryRow[] };
  READBY?: { where: RepositoryRow; expect?: RepositoryRow | null };
  UPDATE?: { row: RepositoryRow; expect?: AffectedExpect };
  UPDATEBY?: { row: RepositoryRow; where: RepositoryRow; expect?: AffectedExpect };
  DELETE?: { id: Scalar; expect?: AffectedExpect };
  DELETEBY?: { where: RepositoryRow; expect?: AffectedExpect };
};

export type RepositorySpec = {
  name: string;
  type: RepositoryTypeSpec;
  steps: RepositoryStep[];
};

const idsOf = (spec: RepositoryTypeSpec): Set<string> => new Set(spec.ids);

const columnsOf = (spec: RepositoryTypeSpec): string[] =>
  Object.keys(spec.fields).sort((left, right) => left.localeCompare(right));

export const flattenRepositoryRow = (
  row: RepositoryRow,
  mappings: RepositoryMappings = {},
): RepositoryRow => {
  const dest: RepositoryRow = { ...row };
  for (const [nested, cols] of Object.entries(mappings)) {
    const value = dest[nested];
    if (value && typeof value === "object") {
      for (const [field, col] of Object.entries(cols)) dest[col] = value[field];
    }
    delete dest[nested];
  }
  return dest;
};

const fieldsOf = (row: RepositoryRow, include: (col: string) => boolean): Field[] =>
  Object.entries(row)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value: value as Scalar }));

const andAll = (parts: IExpression[]) =>
  new Where(
    parts.length === 0
      ? new Operator.Equals(new Literal(1), new Literal(1))
      : parts.reduce((left, right) => new Operator.And(left, right)),
  );

const whereOf = (row: RepositoryRow) =>
  andAll(Object.entries(row).map(([name, value]) => new Operator.Equals(name, value)));

const idWhere = (spec: RepositoryTypeSpec, id: Scalar) =>
  andAll(spec.ids.map((col) => new Operator.Equals(col, id)));

export const physicalTableOf = (type: RepositoryTypeSpec): string =>
  type.crudMapper?.entity ?? type.table;

const withStatementSql = (sql: string, write: boolean, type: RepositoryTypeSpec): string =>
  `${sql}${
    write
      ? (type.crudMapper?.writeSuffix ?? "")
      : (type.crudMapper?.querySuffix ?? "")
  }`;

export const repositoryQueries = (spec: RepositorySpec): string[] => {
  const { type } = spec;
  const ids = idsOf(type);
  const columns = columnsOf(type);
  const table = physicalTableOf(type);
  return spec.steps.map((step) => {
    if (step.CREATE) {
      const fields = fieldsOf(
        flattenRepositoryRow(step.CREATE.row, type.mappings),
        (col) => !ids.has(col),
      );
      return withStatementSql(new InsertStatement(table, fields).getText(), true, type);
    }
    if (step.READ)
      return withStatementSql(
        new SelectStatement({ table, fields: columns, where: idWhere(type, step.READ.id) }).getText(),
        false,
        type,
      );
    if (step.QUERY)
      return withStatementSql(
        new SelectStatement({
          table,
          fields: columns,
          where: step.QUERY.where ? whereOf(step.QUERY.where) : undefined,
        }).getText(),
        false,
        type,
      );
    if (step.READBY)
      return withStatementSql(
        new SelectStatement({ table, fields: columns, where: whereOf(step.READBY.where) }).getText(),
        false,
        type,
      );
    if (step.UPDATE) {
      const flat = flattenRepositoryRow(step.UPDATE.row, type.mappings);
      const fields = fieldsOf(flat, (col) => !ids.has(col));
      return withStatementSql(
        new UpdateStatement(table, fields, idWhere(type, flat.id as Scalar)).getText(),
        true,
        type,
      );
    }
    if (step.UPDATEBY) {
      const fields = fieldsOf(
        flattenRepositoryRow(step.UPDATEBY.row, type.mappings),
        (col) => !ids.has(col),
      );
      return withStatementSql(
        new UpdateStatement(table, fields, whereOf(step.UPDATEBY.where)).getText(),
        true,
        type,
      );
    }
    if (step.DELETE)
      return withStatementSql(
        new DeleteStatement(table, idWhere(type, step.DELETE.id)).getText(),
        true,
        type,
      );
    if (step.DELETEBY)
      return withStatementSql(
        new DeleteStatement(table, whereOf(step.DELETEBY.where)).getText(),
        true,
        type,
      );
    throw new Error(`${spec.name}: empty step`);
  });
};

export const expectedRepositoryConsoleSignatures = (spec: RepositorySpec): string[] =>
  expectedConsoleSignatures({
    steps: repositoryQueries(spec).map((sql) => ({ execute: { sql } })),
  });
