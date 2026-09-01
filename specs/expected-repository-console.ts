import { StatementBuilder, WhereBuilder, type Field, type Scalar } from "@datasources/core";
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

export type RepositoryTypeSpec = {
  name: string;
  table: string;
  ids: string[];
  assignIdentityOnAdd: boolean;
  fields: Record<string, CanonicalType>;
};

export type RepositoryRow = Record<string, Scalar>;

export type AffectedExpect = { rowsAffected: number };

export type RepositoryStep = {
  CREATE?: { row: RepositoryRow };
  GET?: { id: Scalar; expect?: RepositoryRow | null };
  QUERY?: { where?: RepositoryRow; expect?: RepositoryRow[] };
  GETBY?: { where: RepositoryRow; expect?: RepositoryRow | null };
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

const fieldsOf = (row: RepositoryRow, include: (col: string) => boolean): Field[] =>
  Object.entries(row)
    .filter(([key]) => include(key))
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => ({ name, value }));

const whereOf = (row: RepositoryRow) =>
  WhereBuilder.and(
    ...Object.entries(row).map(([name, value]) => WhereBuilder.equals(name, value)),
  );

const idWhere = (spec: RepositoryTypeSpec, id: Scalar) =>
  WhereBuilder.and(...spec.ids.map((col) => WhereBuilder.equals(col, id)));

export const repositoryQueries = (spec: RepositorySpec): string[] => {
  const { type } = spec;
  const ids = idsOf(type);
  const columns = columnsOf(type);
  const table = type.table;
  return spec.steps.map((step) => {
    if (step.CREATE) {
      const fields = fieldsOf(
        step.CREATE.row,
        (col) => !type.assignIdentityOnAdd || !ids.has(col),
      );
      return StatementBuilder.createInsert(table, fields).getText();
    }
    if (step.GET) return StatementBuilder.createQuery(table, columns, idWhere(type, step.GET.id)).getText();
    if (step.QUERY)
      return StatementBuilder.createQuery(
        table,
        columns,
        step.QUERY.where ? whereOf(step.QUERY.where) : undefined,
      ).getText();
    if (step.GETBY)
      return StatementBuilder.createQuery(table, columns, whereOf(step.GETBY.where)).getText();
    if (step.UPDATE) {
      const fields = fieldsOf(step.UPDATE.row, (col) => !ids.has(col));
      return StatementBuilder.createUpdate(table, fields, idWhere(type, step.UPDATE.row.id as Scalar)).getText();
    }
    if (step.UPDATEBY) {
      const fields = fieldsOf(step.UPDATEBY.row, (col) => !ids.has(col));
      return StatementBuilder.createUpdate(table, fields, whereOf(step.UPDATEBY.where)).getText();
    }
    if (step.DELETE)
      return StatementBuilder.createDelete(table, idWhere(type, step.DELETE.id)).getText();
    if (step.DELETEBY) return StatementBuilder.createDelete(table, whereOf(step.DELETEBY.where)).getText();
    throw new Error(`${spec.name}: empty step`);
  });
};

export const expectedRepositoryConsoleSignatures = (spec: RepositorySpec): string[] =>
  expectedConsoleSignatures({
    steps: repositoryQueries(spec).map((sql) => ({ execute: { sql } })),
  });
