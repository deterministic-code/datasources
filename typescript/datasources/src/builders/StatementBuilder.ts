import {
  CreateTableStatement,
  DeleteStatement,
  InsertStatement,
  Query,
  StoredProcedureStatement,
  UpdateStatement,
  type ICreateTableStatement,
  type IDeleteStatement,
  type IInsertStatement,
  type IQuery,
  type IStoredProcedureStatement,
  type IUpdateStatement,
  type IWhere,
} from "../statements/index.ts";
import type { Column } from "./Column.ts";
import type { Field } from "./Field.ts";
import type { Join, JoinKind } from "./Join.ts";

const names = (fields: Field[]): string =>
  fields.map((field) => field.name).join(", ");
const placeholders = (fields: Field[]): string =>
  fields.map((field) => `$${field.name}`).join(", ");
const assignments = (fields: Field[]): string =>
  fields.map((field) => `${field.name} = $${field.name}`).join(", ");
const withWhere = (sql: string, where?: IWhere): string =>
  where ? `${sql} WHERE ${where.getText()}` : sql;

const selectList = (fields: string[]): string =>
  fields.length === 0 ? "*" : fields.join(", ");

const joinKeyword: Record<JoinKind, string> = {
  inner: "INNER JOIN",
  left: "LEFT JOIN",
  right: "RIGHT JOIN",
};

const joinSql = (joins: Join[]): string =>
  joins
    .map((join) => ` ${joinKeyword[join.kind]} ${join.table} ON ${join.left} = ${join.right}`)
    .join("");

export const StatementBuilder = {
  createQuery: (
    table: string,
    fields: string[],
    where?: IWhere,
    joins: Join[] = [],
  ): IQuery =>
    new Query(
      withWhere(`SELECT ${selectList(fields)} FROM ${table}${joinSql(joins)}`, where),
      where,
    ),
  createInsert: (table: string, fields: Field[]): IInsertStatement =>
    new InsertStatement(
      `INSERT INTO ${table} (${names(fields)}) VALUES (${placeholders(fields)})`,
    ),
  createUpdate: (
    table: string,
    fields: Field[],
    where?: IWhere,
  ): IUpdateStatement =>
    new UpdateStatement(
      withWhere(`UPDATE ${table} SET ${assignments(fields)}`, where),
      where,
    ),
  createDelete: (table: string, where?: IWhere): IDeleteStatement =>
    new DeleteStatement(withWhere(`DELETE FROM ${table}`, where), where),
  createTable: (table: string, columns: Column[]): ICreateTableStatement =>
    new CreateTableStatement(
      `CREATE TABLE ${table} (${columns.map((column) => `${column.name} ${column.type}`).join(", ")})`,
    ),
  createStoredProcedure: (
    name: string,
    parameters: Field[] = [],
  ): IStoredProcedureStatement =>
    new StoredProcedureStatement(`CALL ${name}(${placeholders(parameters)})`),
};
