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
  type StatementNaming,
} from "@deterministic-code/datasources";
import type { DatasourceIndex, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { isPkField } from "@deterministic-code/generators-common/spec-types";
import { converterFieldOf } from "./common/converter-field.ts";
import {
  fieldOverlay,
  fieldStem,
  overlayOf,
  resolveReference,
  tableIdentity,
  tableStem,
  usesGeneratedIdColumn,
  type LiveTable,
  type SqlTable,
} from "./common/sql-schema.ts";
export const columnOf = (naming: StatementNaming, table: LiveTable, field: TypeField): Column => {
  const converterField = converterFieldOf(field);
  const generated = usesGeneratedIdColumn(table, field);
  const named = UniqueConstraint.nameOf(
    tableStem(table), naming, fieldStem(table, field.name), "default_constraint",
  );
  const fromSpec = converterField.defaultValue;
  const identity = generated && fromSpec == null;
  const keys = tableIdentity(table);
  return {
    name: CreateTableStatement.columnName(fieldStem(table, field.name), naming),
    type: converterField.type,
    ...(fromSpec != null ? { default: new ColumnDefault(String(fromSpec), named) } : {}),
    ...(!field.isNullable || keys.includes(field.name) ? { notNull: new NotNull() } : {}),
    ...(identity ? { identity: true } : {}),
    field: converterField,
  };
};

export const createTableStatementOf = (
  table: LiveTable,
  naming: StatementNaming,
  tables: readonly SqlTable[],
): CreateTableStatement => {
  const keys = tableIdentity(table);
  const columns = table.fields.map((field) => columnOf(naming, table, field));
  const stem = tableStem(table);
  const uniques = table.fields
    .filter((field) => fieldOverlay(table, field.name)?.isUnique === true || field.name === "uuid")
    .map(
      (field) =>
        new UniqueConstraint(
          UniqueConstraint.nameOf(stem, naming, fieldStem(table, field.name), "unique_constraint"),
          [CreateTableStatement.columnName(fieldStem(table, field.name), naming)],
        ),
    );
  const foreignKeys = table.fields
    .filter((field) => field.references !== undefined)
    .map((field) => foreignKeyOf(naming, tables, table, field));
  const pk =
    keys.length > 1
      ? [new PrimaryKey(PrimaryKey.nameOf(stem, naming, "primary_key"), keys.map((key) => CreateTableStatement.columnName(fieldStem(table, key), naming)))]
      : keys.length === 1 && table.fields.some((field) => isPkField(field, table, overlayOf(table)))
        ? [new PrimaryKey(PrimaryKey.nameOf(stem, naming, "primary_key"), [CreateTableStatement.columnName(fieldStem(table, keys[0]!), naming)])]
        : [];
  return new CreateTableStatement(table.tableName, columns, [...pk, ...uniques, ...foreignKeys]);
};

export const createIndexStatementOf = (
  naming: StatementNaming,
  table: LiveTable,
  idx: DatasourceIndex,
): CreateIndexStatement =>
  new CreateIndexStatement(
    CreateIndexStatement.nameOf(tableStem(table), naming, idx.name, "index"),
    table.tableName,
    idx.fields.map((column) => CreateTableStatement.columnName(fieldStem(table, column), naming)),
    idx.isUnique,
  );

export const dropTableStatementOf = (table: string): DropTableStatement =>
  new DropTableStatement(table);

export const dropIndexStatementOf = (name: string, table: string): DropIndexStatement =>
  new DropIndexStatement(name, table);

export const dropTriggerStatementOf = (name: string, table: string): DropTriggerStatement =>
  new DropTriggerStatement(name, table);

export const uniqueConstraintOf = (
  naming: StatementNaming,
  table: LiveTable,
  field: TypeField,
): UniqueConstraint =>
  new UniqueConstraint(
    UniqueConstraint.nameOf(tableStem(table), naming, fieldStem(table, field.name), "unique_constraint"),
    [CreateTableStatement.columnName(fieldStem(table, field.name), naming)],
  );

export const foreignKeyOf = (
  naming: StatementNaming,
  tables: readonly SqlTable[],
  table: LiveTable,
  field: TypeField,
): ForeignKey => {
  const ref = resolveReference(tables, naming, field);
  return new ForeignKey(
    ForeignKey.nameOf(tableStem(table), naming, fieldStem(table, field.name), "foreign_key"),
    [CreateTableStatement.columnName(fieldStem(table, field.name), naming)],
    ref.tableName,
    [ref.columnName],
  );
};

export const addForeignKeyStatementOf = (
  naming: StatementNaming,
  tables: readonly SqlTable[],
  table: LiveTable,
  field: TypeField,
): ForeignKeyStatement => {
  const fk = foreignKeyOf(naming, tables, table, field);
  return new ForeignKeyStatement(table.tableName, fk.name, fk.columns, fk.referencesTable, fk.referencesColumns);
};

export const dropForeignKeyStatementOf = (
  naming: StatementNaming,
  tables: readonly SqlTable[],
  table: LiveTable,
  field: TypeField,
): ForeignKeyStatement =>
  new ForeignKeyStatement(table.tableName, foreignKeyOf(naming, tables, table, field).name, [], "", [], "drop");

export const createTriggerStatementOf = (
  table: LiveTable,
  naming: StatementNaming,
  key: string,
): CreateTriggerStatement =>
  new CreateTriggerStatement(
    table.tableName,
    CreateTriggerStatement.nameOf(tableStem(table), naming),
    CreateTableStatement.columnName(fieldStem(table, "updated"), naming),
    CreateTableStatement.columnName(fieldStem(table, key), naming),
  );

export const seedInsertStatementOf = (
  table: LiveTable,
  naming: StatementNaming,
  columns: string[],
  values: string[],
): InsertStatement =>
  new InsertStatement(
    table.tableName,
    columns.map((column, i) => ({
      name: CreateTableStatement.columnName(fieldStem(table, column), naming),
      value: values[i] ?? "NULL",
    })),
  );

export const identityInsertStatementOf = (
  table: LiveTable,
  naming: StatementNaming,
  inserts: InsertStatement[],
  identity: boolean,
): IdentityInsertStatement => {
  const keys = tableIdentity(table);
  return new IdentityInsertStatement(
    table.tableName,
    CreateTableStatement.columnName(fieldStem(table, keys[0] ?? "id"), naming),
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
