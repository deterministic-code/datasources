import { isDeepStrictEqual } from "node:util";
import {
  AlterTableStatement,
  type Column,
  type IStatement,
} from "@deterministic-code/datasources";
import type { DatasourceIndex, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import {
  createIndexStatementOf,
  createTableStatementOf,
  createTriggerStatementOf,
  columnOf,
  dropIndexStatementOf,
  dropTableStatementOf,
  dropTriggerStatementOf,
  addForeignKeyStatementOf,
  dropForeignKeyStatementOf,
  uniqueConstraintOf,
} from "../statements.ts";
import type { SqlMapping } from "./sql-mapping.ts";
import { dialectDriver, type SqlDialect } from "./sql-dialect.ts";
import {
  fieldOverlay,
  hasAuditColumns,
  isGeneratedIdentity,
  overlayOf,
  tableIdentity,
  type LiveTable,
} from "./sql-schema.ts";
import { pkName } from "@deterministic-code/generators-common/spec-types";

export type SchemaSnapshot = {
  tables: LiveTable[];
  mapping: SqlMapping;
};

const byName = (tables: LiveTable[]): Map<string, LiveTable> =>
  new Map(tables.map((table) => [table.name, table]));

const fieldByName = (table: LiveTable): Map<string, TypeField> =>
  new Map(table.fields.map((field) => [field.name, field]));

const indexKey = (index: DatasourceIndex): string =>
  JSON.stringify([index.name, index.fields, index.isUnique]);

const uniqueField = (table: LiveTable, field: TypeField): boolean =>
  fieldOverlay(table, field.name)?.isUnique === true || field.name === "uuid";

const refsOf = (field: TypeField): string => JSON.stringify(field.references ?? null);

const identityOf = (table: LiveTable): unknown => [
  tableIdentity(table),
  isGeneratedIdentity(table),
];

const trigger = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
): IStatement =>
  createTriggerStatementOf(table, mapping, pkName(table, overlayOf(table)) ?? "id");

const dropTrigger = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
): IStatement =>
  dropTriggerStatementOf(mapping.triggerName(table), table.tableName);

const dropIndex = (
  dialect: SqlDialect,
  mapping: SqlMapping,
  table: LiveTable,
  index: DatasourceIndex,
): IStatement =>
  dropIndexStatementOf(
    mapping.constraintName(table, index.name, "index"),
    table.tableName,
  );

const compiledColumn = (dialect: SqlDialect, column: Column, name: string) => {
  const ctor = dialectDriver(dialect).constructor as {
    nativeType?: (column: Column) => string;
  };
  return {
    name,
    type: ctor.nativeType?.(column) ?? column.type,
    default: column.default,
    notNull: column.notNull,
    identity: column.identity,
  };
};

const createTable = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
): IStatement[] => [
  createTableStatementOf(table, mapping),
  ...table.indexes.map((index) => createIndexStatementOf(mapping, table, index)),
  ...(hasAuditColumns(table) ? [trigger(dialect, table, mapping)] : []),
];

const dropTable = (
  dialect: SqlDialect,
  table: LiveTable,
  mapping: SqlMapping,
): IStatement[] => [
  ...(hasAuditColumns(table) ? [dropTrigger(dialect, table, mapping)] : []),
  ...[...table.indexes].reverse().map((index) => dropIndex(dialect, mapping, table, index)),
  dropTableStatementOf(table.tableName),
];

const diffTable = (
  dialect: SqlDialect,
  before: LiveTable,
  after: LiveTable,
  beforeMap: SqlMapping,
  afterMap: SqlMapping,
): IStatement[] => {
  if (!isDeepStrictEqual(identityOf(before), identityOf(after))) {
    throw new Error(`${after.name}: primary key or identity change is not supported`);
  }
  const beforeFields = fieldByName(before);
  const afterFields = fieldByName(after);
  const droppedFields = before.fields.filter((field) => !afterFields.has(field.name));
  const addedFields = after.fields.filter((field) => !beforeFields.has(field.name));
  const keptFields = after.fields.filter((field) => beforeFields.has(field.name));
  const beforeIndexes = new Map(before.indexes.map((index) => [index.name, index]));
  const afterIndexes = new Map(after.indexes.map((index) => [index.name, index]));
  const out: IStatement[] = [];

  for (const index of before.indexes) {
    const next = afterIndexes.get(index.name);
    if (next === undefined || indexKey(index) !== indexKey(next)) {
      out.push(dropIndex(dialect, beforeMap, before, index));
    }
  }

  for (const field of before.fields) {
    if (uniqueField(before, field) && (!afterFields.has(field.name) || !uniqueField(after, afterFields.get(field.name)!))) {
      out.push(
        new AlterTableStatement(before.tableName, {
          kind: "dropConstraint",
          name: uniqueConstraintOf(beforeMap, before, field).name,
        }),
      );
    }
    if (field.references !== undefined && (!afterFields.has(field.name) || refsOf(afterFields.get(field.name)!) !== refsOf(field))) {
      out.push(dropForeignKeyStatementOf(beforeMap, before, field));
    }
  }

  if (hasAuditColumns(before) && !hasAuditColumns(after)) {
    out.push(dropTrigger(dialect, before, beforeMap));
  }

  for (const field of droppedFields) {
    out.push(
      new AlterTableStatement(before.tableName, {
        kind: "dropColumn",
        name: columnOf(beforeMap, before, field).name,
      }),
    );
  }

  const tableName = after.tableName;
  if (before.tableName !== after.tableName) {
    out.push(new AlterTableStatement(before.tableName, { kind: "renameTable", to: after.tableName }));
  }

  for (const field of keptFields) {
    const previous = columnOf(beforeMap, before, beforeFields.get(field.name)!);
    const next = columnOf(afterMap, after, field);
    if (previous.name !== next.name) {
      out.push(new AlterTableStatement(tableName, { kind: "renameColumn", from: previous.name, to: next.name }));
    }
    if (!isDeepStrictEqual(compiledColumn(dialect, previous, next.name), compiledColumn(dialect, next, next.name))) {
      out.push(new AlterTableStatement(tableName, { kind: "alterColumn", column: next }));
    }
  }

  for (const field of addedFields) {
    out.push(
      new AlterTableStatement(tableName, {
        kind: "addColumn",
        column: columnOf(afterMap, after, field),
      }),
    );
  }

  for (const field of after.fields) {
    if (uniqueField(after, field) && (!beforeFields.has(field.name) || !uniqueField(before, beforeFields.get(field.name)!))) {
      out.push(
        new AlterTableStatement(tableName, {
          kind: "addConstraint",
          constraint: uniqueConstraintOf(afterMap, after, field),
        }),
      );
    }
    if (field.references !== undefined && (!beforeFields.has(field.name) || refsOf(beforeFields.get(field.name)!) !== refsOf(field))) {
        out.push(addForeignKeyStatementOf(afterMap, after, field));
    }
  }

  for (const index of after.indexes) {
    const previous = beforeIndexes.get(index.name);
    if (previous === undefined || indexKey(previous) !== indexKey(index)) {
      out.push(createIndexStatementOf(afterMap, after, index));
    }
  }

  if (!hasAuditColumns(before) && hasAuditColumns(after)) {
    out.push(trigger(dialect, after, afterMap));
  }

  return out;
};

/** Ordered CREATE / ALTER / DROP statements that take `before` to `after`. */
export const schemaDiff = (
  dialect: SqlDialect,
  before: SchemaSnapshot,
  after: SchemaSnapshot,
): IStatement[] => {
  const previous = byName(before.tables);
  const next = byName(after.tables);
  const removed = before.tables.filter((table) => !next.has(table.name));
  const added = after.tables.filter((table) => !previous.has(table.name));
  const kept = after.tables.filter((table) => previous.has(table.name));
  return [
    ...[...removed].reverse().flatMap((table) => dropTable(dialect, table, before.mapping)),
    ...kept.flatMap((table) =>
      diffTable(dialect, previous.get(table.name)!, table, before.mapping, after.mapping),
    ),
    ...added.flatMap((table) => createTable(dialect, table, after.mapping)),
  ];
};
