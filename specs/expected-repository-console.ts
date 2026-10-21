import {
  DefaultStatementTextRenderer,
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
} from "@deterministic-code/datasources";
import { expectedConsoleSignatures } from "./expected-console-output.ts";

const render = (statement: InsertStatement | SelectStatement | UpdateStatement | DeleteStatement): string =>
  DefaultStatementTextRenderer.instance.render(statement);

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
  schema: string;
  type: RepositoryTypeSpec;
  steps: RepositoryStep[];
};

const idsOf = (spec: RepositoryTypeSpec): Set<string> => new Set(spec.ids);

const assigned = (value: Scalar | RepositoryRow | undefined): boolean =>
  value !== null && value !== undefined && value !== "" && value !== 0 && typeof value !== "object";

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

const withStatementSql = (sql: string, _write: boolean, _type: RepositoryTypeSpec): string => sql;

const insertedIdSql = (provider: string, table: string, column: string): string =>
  ({
    sqlite: `SELECT last_insert_rowid() AS ${column}`,
    mysql: `SELECT LAST_INSERT_ID() AS ${column}`,
    postgres: `SELECT setval(pg_get_serial_sequence('${table}', '${column}'), COALESCE((SELECT MAX("${column}") FROM ${table}), 1)) AS ${column}`,
  })[provider] ?? "";

export const repositoryQueries = (spec: RepositorySpec, provider = ""): string[] => {
  const { type } = spec;
  const ids = idsOf(type);
  const columns = columnsOf(type);
  const table = physicalTableOf(type);
  return spec.steps.map((step) => {
    if (step.CREATE) {
      const flat = flattenRepositoryRow(step.CREATE.row, type.mappings);
      const fields = fieldsOf(flat, (col) => !ids.has(col) || assigned(flat[col]));
      return withStatementSql(render(new InsertStatement(table, fields)), true, type);
    }
    if (step.READ)
      return withStatementSql(
        render(new SelectStatement({ table, fields: columns, where: idWhere(type, step.READ.id) })),
        false,
        type,
      );
    if (step.QUERY)
      return withStatementSql(
        render(
          new SelectStatement({
            table,
            fields: columns,
            where: step.QUERY.where ? whereOf(step.QUERY.where) : undefined,
          }),
        ),
        false,
        type,
      );
    if (step.READBY)
      return withStatementSql(
        render(new SelectStatement({ table, fields: columns, where: whereOf(step.READBY.where) })),
        false,
        type,
      );
    if (step.UPDATE) {
      const flat = flattenRepositoryRow(step.UPDATE.row, type.mappings);
      const fields = fieldsOf(flat, (col) => !ids.has(col));
      return withStatementSql(
        render(new UpdateStatement(table, fields, idWhere(type, flat.id as Scalar))),
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
        render(new UpdateStatement(table, fields, whereOf(step.UPDATEBY.where))),
        true,
        type,
      );
    }
    if (step.DELETE)
      return withStatementSql(
        render(new DeleteStatement(table, idWhere(type, step.DELETE.id))),
        true,
        type,
      );
    if (step.DELETEBY)
      return withStatementSql(
        render(new DeleteStatement(table, whereOf(step.DELETEBY.where))),
        true,
        type,
      );
    throw new Error(`${spec.name}: empty step`);
  }).flatMap((sql, index) => {
    const step = spec.steps[index];
    const id = spec.type.ids[0] ?? "id";
    const inserted =
      step?.CREATE && spec.type.ids.length === 1
      && !assigned(flattenRepositoryRow(step.CREATE.row, type.mappings)[id])
        ? insertedIdSql(provider, table, id)
        : "";
    return inserted ? [sql, inserted] : [sql];
  });
};

export const expectedRepositoryConsoleSignatures = (spec: RepositorySpec, provider = ""): string[] =>
  expectedConsoleSignatures({
    steps: repositoryQueries(spec, provider).map((sql) => ({ execute: { sql } })),
  });
