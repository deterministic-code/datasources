import {
  ColumnDefault,
  CreateBulkCreateStoredProcedureStatement,
  CreateBulkDeleteStoredProcedureStatement,
  CreateBulkUpdateStoredProcedureStatement,
  CreateCreateStoredProcedureStatement,
  CreateDeleteStoredProcedureStatement,
  CreateIndexStatement,
  CreateReadStoredProcedureStatement,
  CreateTableStatement,
  CreateTriggerStatement,
  CreateUpdateStoredProcedureStatement,
  DeleteStatement,
  DropIndexStatement,
  DropRoutineStatement,
  DropTableStatement,
  DropTriggerStatement,
  ForeignKey,
  IdentityInsertStatement,
  InsertStatement,
  NotNull,
  PrimaryKey,
  SelectStatement,
  UniqueConstraint,
  UpdateStatement,
  Operator,
  Where,
  eq,
  type Column,
  type RoutineParameter,
} from "@deterministic-code/datasources";
import type { DatasourceIndex, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { isPkField } from "@deterministic-code/generators-common/spec-types";
import { converterFieldOf } from "./common/converter-field.ts";
import {
  fieldOverlay,
  overlayOf,
  tableIdentity,
  usesGeneratedIdColumn,
  type LiveTable,
} from "./common/sql-schema.ts";
import type { SqlMapping } from "./common/sql-mapping.ts";
import {
  dialectDriver,
  mapColumnType,
  q,
  sqlDefault,
  type SqlDialect,
} from "./common/sql-dialect.ts";

export const columnOf = (dialect: SqlDialect, mapping: SqlMapping, table: LiveTable, field: TypeField): Column => {
  const overlay = fieldOverlay(table, field.name);
  const converterField = converterFieldOf(field);
  const generated = usesGeneratedIdColumn(table, field);
  const named = mapping.constraintName(table, mapping.fieldStem(table, field.name), "default_constraint");
  const fromSpec = sqlDefault(dialect, converterField);
  const identity = generated && fromSpec == null;
  const keys = tableIdentity(table);
  return {
    name: q(dialect, mapping.columnName(table, field.name)),
    type: mapColumnType(dialect, converterField),
    ...(fromSpec ? { default: new ColumnDefault(fromSpec, q(dialect, named)) } : {}),
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
          q(dialect, mapping.constraintName(table, mapping.fieldStem(table, field.name), "unique_constraint")),
          [q(dialect, mapping.columnName(table, field.name))],
        ),
    );
  const foreignKeys = table.fields
    .filter((field) => field.references !== undefined)
    .map((field) => {
      const ref = mapping.resolveReference(field.references!);
      return new ForeignKey(
        q(dialect, mapping.constraintName(table, mapping.fieldStem(table, field.name), "foreign_key")),
        [q(dialect, mapping.columnName(table, field.name))],
        q(dialect, ref.tableName),
        [q(dialect, ref.columnName)],
      );
    });
  const pk =
    keys.length > 1
      ? [new PrimaryKey(q(dialect, mapping.constraintName(table, "primary_key")), keys.map((key) => q(dialect, mapping.columnName(table, key))))]
      : keys.length === 1 && table.fields.some((field) => isPkField(field, table, overlayOf(table)))
        ? [new PrimaryKey(q(dialect, mapping.constraintName(table, "primary_key")), [q(dialect, mapping.columnName(table, keys[0]!))])]
        : [];
  return new CreateTableStatement(q(dialect, table.tableName), columns, [...pk, ...uniques, ...foreignKeys]);
};

export const createIndexStatementOf = (
  dialect: SqlDialect,
  mapping: SqlMapping,
  table: LiveTable,
  idx: DatasourceIndex,
): CreateIndexStatement =>
  new CreateIndexStatement(
    q(dialect, mapping.constraintName(table, idx.name, "index")),
    q(dialect, table.tableName),
    idx.fields.map((column) => q(dialect, mapping.columnName(table, column))),
    idx.isUnique,
  );

export const dropTableStatementOf = (dialect: SqlDialect, table: string): DropTableStatement =>
  new DropTableStatement(q(dialect, table));

export const dropIndexStatementOf = (
  dialect: SqlDialect,
  name: string,
  table: string,
): DropIndexStatement => new DropIndexStatement(q(dialect, name), q(dialect, table));

export const dropTriggerStatementOf = (
  dialect: SqlDialect,
  name: string,
  table: string,
): DropTriggerStatement => new DropTriggerStatement(q(dialect, name), q(dialect, table));

export const uniqueConstraintOf = (
  dialect: SqlDialect,
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): UniqueConstraint =>
  new UniqueConstraint(
    q(dialect, mapping.constraintName(table, mapping.fieldStem(table, field.name), "unique_constraint")),
    [q(dialect, mapping.columnName(table, field.name))],
  );

export const foreignKeyOf = (
  dialect: SqlDialect,
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): ForeignKey => {
  const ref = mapping.resolveReference(field.references!);
  return new ForeignKey(
    q(dialect, mapping.constraintName(table, mapping.fieldStem(table, field.name), "foreign_key")),
    [q(dialect, mapping.columnName(table, field.name))],
    q(dialect, ref.tableName),
    [q(dialect, ref.columnName)],
  );
};

export const createTriggerStatementOf = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
  key: string,
): CreateTriggerStatement =>
  new CreateTriggerStatement(
    q(dialect, table.tableName),
    q(dialect, mapping.triggerName(table)),
    q(dialect, mapping.columnName(table, "updated")),
    q(dialect, mapping.columnName(table, key)),
    dialectDriver(dialect).defaultValueConverter.convert("datetime", "UtcNow") ?? "",
  );

export const seedInsertStatementOf = (
  table: LiveTable,
  mapping: SqlMapping,
  columns: string[],
  values: string[],
  dialect: SqlDialect,
): InsertStatement =>
  new InsertStatement(
    q(dialect, table.tableName),
    columns.map((column, i) => ({
      name: q(dialect, mapping.columnName(table, column)),
      value: values[i] ?? "NULL",
    })),
  );

export const identityInsertStatementOf = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
  inserts: InsertStatement[],
  identity: boolean,
): IdentityInsertStatement => {
  const keys = tableIdentity(table);
  return new IdentityInsertStatement(
    table.tableName,
    mapping.columnName(table, keys[0] ?? "id"),
    inserts,
    identity,
  );
};

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

const equalsWhere = ([first, ...rest]: string[]): Where =>
  new Where(
    rest.reduce(
      (condition, column) => new Operator.And(condition, eq(column, column)),
      eq(first!, first!),
    ),
  );

export const updateStatementOf = (
  table: string,
  fields: string[],
  keys: string[],
): UpdateStatement =>
  new UpdateStatement(
    table,
    fields.map((name) => ({ name, value: name })),
    equalsWhere(keys),
  );

export const deleteStatementOf = (table: string, keys: string[]): DeleteStatement =>
  new DeleteStatement(table, equalsWhere(keys));

export const dropRoutineStatementOf = (name: string): DropRoutineStatement =>
  new DropRoutineStatement(name);

const rowArgs = (
  name: string,
  table: string,
  columns: readonly string[],
  parameters: readonly RoutineParameter[],
  key: string,
  keyType: string,
  writable: readonly string[],
  generatedKey = false,
  auditUpdated?: string,
) =>
  [name, table, columns, parameters, key, keyType, writable, generatedKey, auditUpdated] as const;

export const createCreateStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly string[],
  parameters: readonly RoutineParameter[],
  key: string,
  keyType: string,
  writable: readonly string[],
  generatedKey = false,
  auditUpdated?: string,
): CreateCreateStoredProcedureStatement =>
  new CreateCreateStoredProcedureStatement(...rowArgs(name, table, columns, parameters, key, keyType, writable, generatedKey, auditUpdated));

export const createReadStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly string[],
  parameters: readonly RoutineParameter[],
  key: string,
  keyType: string,
  writable: readonly string[],
  byProperties: readonly string[] = [],
  generatedKey = false,
  auditUpdated?: string,
): CreateReadStoredProcedureStatement =>
  new CreateReadStoredProcedureStatement(name, table, columns, parameters, key, keyType, writable, byProperties, generatedKey, auditUpdated);

export const createUpdateStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly string[],
  parameters: readonly RoutineParameter[],
  key: string,
  keyType: string,
  writable: readonly string[],
  byProperties: readonly string[] = [],
  generatedKey = false,
  auditUpdated?: string,
): CreateUpdateStoredProcedureStatement =>
  new CreateUpdateStoredProcedureStatement(name, table, columns, parameters, key, keyType, writable, byProperties, generatedKey, auditUpdated);

export const createDeleteStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly string[],
  parameters: readonly RoutineParameter[],
  key: string,
  keyType: string,
  writable: readonly string[],
  byProperties: readonly string[] = [],
  generatedKey = false,
  auditUpdated?: string,
): CreateDeleteStoredProcedureStatement =>
  new CreateDeleteStoredProcedureStatement(name, table, columns, parameters, key, keyType, writable, byProperties, generatedKey, auditUpdated);

export const createBulkCreateStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly RoutineParameter[],
  payload: RoutineParameter,
  key: string,
  keyType: string,
): CreateBulkCreateStoredProcedureStatement =>
  new CreateBulkCreateStoredProcedureStatement(name, table, columns, payload, key, keyType);

export const createBulkUpdateStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly RoutineParameter[],
  payload: RoutineParameter,
  key: string,
  keyType: string,
): CreateBulkUpdateStoredProcedureStatement =>
  new CreateBulkUpdateStoredProcedureStatement(name, table, columns, payload, key, keyType);

export const createBulkDeleteStoredProcedureOf = (
  name: string,
  table: string,
  columns: readonly RoutineParameter[],
  payload: RoutineParameter,
  key: string,
  keyType: string,
): CreateBulkDeleteStoredProcedureStatement =>
  new CreateBulkDeleteStoredProcedureStatement(name, table, columns, payload, key, keyType);
