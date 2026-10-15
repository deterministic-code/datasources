import {
  ColumnDefault,
  CreateIndexStatement,
  CreateTableStatement,
  DeleteStatement,
  ForeignKey,
  InsertStatement,
  NotNull,
  PrimaryKey,
  SelectStatement,
  UniqueConstraint,
  UpdateStatement,
  Where,
  eq,
  type Column,
} from "@datasources/core";
import type { DatasourceIndex, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { isPkField } from "@deterministic-code/generators-common/spec-types";
import { converterFieldOf } from "./common/occ-sql.ts";
import {
  fieldOverlay,
  overlayOf,
  tableIdentity,
  usesGeneratedIdColumn,
  type LiveTable,
} from "./common/sql-schema.ts";
import type { SqlMapping } from "./common/sql-mapping.ts";
import {
  mapColumnType,
  q,
  sqlDefault,
  type SqlDialect,
} from "./common/sql-dialect.ts";

const columnOf = (dialect: SqlDialect, mapping: SqlMapping, table: LiveTable, field: TypeField): Column => {
  const overlay = fieldOverlay(table, field.name);
  const converterField = converterFieldOf(field, overlay);
  const defaultExpr = sqlDefault(dialect, converterField);
  const keys = tableIdentity(table);
  const identity = usesGeneratedIdColumn(table, field) && field.type !== "uuid" && field.type !== "string";
  return {
    name: mapping.columnName(table, field.name),
    type: mapColumnType(dialect, converterField),
    ...(defaultExpr !== null ? { default: new ColumnDefault(defaultExpr) } : {}),
    ...(!field.isNullable || keys.includes(field.name) ? { notNull: new NotNull() } : {}),
    ...(identity ? { identity: true } : {}),
  };
};

export const createTableStatementOf = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
): CreateTableStatement => {
  const keys = tableIdentity(table);
  const columns = table.fields.map((field) => columnOf(dialect, mapping, table, field));
  const uniques = table.fields
    .filter((field) => fieldOverlay(table, field.name)?.isUnique === true || field.name === "uuid")
    .map(
      (field) =>
        new UniqueConstraint(
          mapping.constraintName(table, mapping.fieldStem(table, field.name), "unique_constraint"),
          [mapping.columnName(table, field.name)],
        ),
    );
  const foreignKeys = table.fields
    .filter((field) => field.references !== undefined)
    .map((field) => {
      const ref = mapping.resolveReference(field.references!);
      return new ForeignKey(
        mapping.constraintName(table, mapping.fieldStem(table, field.name), "foreign_key"),
        [mapping.columnName(table, field.name)],
        ref.tableName,
        [ref.columnName],
      );
    });
  const pk =
    keys.length > 1
      ? [new PrimaryKey(mapping.constraintName(table, "primary_key"), keys.map((key) => mapping.columnName(table, key)))]
      : keys.length === 1 && table.fields.some((field) => isPkField(field, table, overlayOf(table)))
        ? [new PrimaryKey(mapping.constraintName(table, "primary_key"), [mapping.columnName(table, keys[0]!)])]
        : [];
  return new CreateTableStatement(table.tableName, columns, [...pk, ...uniques, ...foreignKeys]);
};

export const createIndexStatementOf = (
  mapping: SqlMapping,
  table: LiveTable,
  idx: DatasourceIndex,
): CreateIndexStatement =>
  new CreateIndexStatement(
    mapping.constraintName(table, idx.name, "index"),
    table.tableName,
    idx.fields.map((column) => mapping.columnName(table, column)),
    idx.isUnique,
  );

export const seedInsertStatementOf = (
  table: LiveTable,
  mapping: SqlMapping,
  columns: string[],
  values: string[],
): InsertStatement =>
  new InsertStatement(
    table.tableName,
    columns.map((column, i) => ({
      name: mapping.columnName(table, column),
      value: values[i] ?? "NULL",
    })),
  );

export const findStatementOf = (
  dialect: SqlDialect,
  table: string,
  fields: string[],
  key: string,
): SelectStatement =>
  new SelectStatement({
    table: q(dialect, table),
    fields: fields.map((field) => `t.${q(dialect, field)}`),
    where: new Where(eq(`t.${q(dialect, key)}`, key)),
  });

export const insertStatementOf = (table: string, fields: string[]): InsertStatement =>
  new InsertStatement(
    table,
    fields.map((name) => ({ name, value: name })),
  );

export const updateStatementOf = (
  table: string,
  fields: string[],
  key: string,
): UpdateStatement =>
  new UpdateStatement(
    table,
    fields.map((name) => ({ name, value: name })),
    new Where(eq(key, key)),
  );

export const deleteStatementOf = (table: string, key: string): DeleteStatement =>
  new DeleteStatement(table, new Where(eq(key, key)));
