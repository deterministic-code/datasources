import { CreateTableStatement } from "./CreateTableStatement.ts";
import { Query } from "../Query.ts";
import type { IQuery } from "../IQuery.ts";
import { DeleteStatement } from "./DeleteStatement.ts";
import { InsertStatement } from "./InsertStatement.ts";
import { StoredProcedureStatement } from "./StoredProcedureStatement.ts";
import { UpdateStatement } from "./UpdateStatement.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";
import type { IDeleteStatement } from "./IDeleteStatement.ts";
import type { IInsertStatement } from "./IInsertStatement.ts";
import type { IStoredProcedureStatement } from "./IStoredProcedureStatement.ts";
import type { IUpdateStatement } from "./IUpdateStatement.ts";
import type { IWhere } from "../IWhere.ts";
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
