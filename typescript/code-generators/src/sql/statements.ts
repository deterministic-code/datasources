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
  ForeignKeyStatement,
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
export const columnOf = (mapping: SqlMapping, table: LiveTable, field: TypeField): Column => {
  const converterField = converterFieldOf(field);
  const generated = usesGeneratedIdColumn(table, field);
  const named = mapping.constraintName(table, mapping.fieldStem(table, field.name), "default_constraint");
  const fromSpec = converterField.defaultValue;
  const identity = generated && fromSpec == null;
  const keys = tableIdentity(table);
  return {
    name: mapping.columnName(table, field.name),
    type: converterField.type,
    ...(fromSpec != null ? { default: new ColumnDefault(String(fromSpec), named) } : {}),
    ...(!field.isNullable || keys.includes(field.name) ? { notNull: new NotNull() } : {}),
    ...(identity ? { identity: true } : {}),
    field: converterField,
  };
};

export const createTableStatementOf = (
  table: LiveTable,
  mapping: SqlMapping,
): CreateTableStatement => {
  const keys = tableIdentity(table);
  const columns = table.fields.map((field) => columnOf(mapping, table, field));
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
    .map((field) => foreignKeyOf(mapping, table, field));
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

export const dropTableStatementOf = (table: string): DropTableStatement =>
  new DropTableStatement(table);

export const dropIndexStatementOf = (name: string, table: string): DropIndexStatement =>
  new DropIndexStatement(name, table);

export const dropTriggerStatementOf = (name: string, table: string): DropTriggerStatement =>
  new DropTriggerStatement(name, table);

export const uniqueConstraintOf = (
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): UniqueConstraint =>
  new UniqueConstraint(
    mapping.constraintName(table, mapping.fieldStem(table, field.name), "unique_constraint"),
    [mapping.columnName(table, field.name)],
  );

export const foreignKeyOf = (
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): ForeignKey => {
  const ref = mapping.resolveReference(field.references!);
  return new ForeignKey(
    mapping.constraintName(table, mapping.fieldStem(table, field.name), "foreign_key"),
    [mapping.columnName(table, field.name)],
    ref.tableName,
    [ref.columnName],
  );
};

export const addForeignKeyStatementOf = (
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): ForeignKeyStatement => {
  const fk = foreignKeyOf(mapping, table, field);
  return new ForeignKeyStatement(table.tableName, fk.name, fk.columns, fk.referencesTable, fk.referencesColumns);
};

export const dropForeignKeyStatementOf = (
  mapping: SqlMapping,
  table: LiveTable,
  field: TypeField,
): ForeignKeyStatement =>
  new ForeignKeyStatement(table.tableName, foreignKeyOf(mapping, table, field).name, [], "", [], "drop");

export const createTriggerStatementOf = (
  table: LiveTable,
  mapping: SqlMapping,
  key: string,
): CreateTriggerStatement =>
  new CreateTriggerStatement(
    table.tableName,
    mapping.triggerName(table),
    mapping.columnName(table, "updated"),
    mapping.columnName(table, key),
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

export const identityInsertStatementOf = (
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
  table: string,
  fields: string[],
  key: string,
): SelectStatement =>
  new SelectStatement({
    table,
    fields: fields.map((field) => `t.${field}`),
    where: new Where(eq(`t.${key}`, key)),
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
