import { dialectDriver } from "./common/sql-dialect.ts";

const sqlite = dialectDriver("sqlite");
const mysql = dialectDriver("mysql");
const postgres = dialectDriver("postgres");
const oracle = dialectDriver("oracle");
const sqlserver = dialectDriver("sqlserver");

export const sqliteTypeConverter = sqlite.typeConverter;
export const sqliteDefaultValueConverter = sqlite.defaultValueConverter;
export const mysqlTypeConverter = mysql.typeConverter;
export const mysqlDefaultValueConverter = mysql.defaultValueConverter;
export const postgresTypeConverter = postgres.typeConverter;
export const postgresDefaultValueConverter = postgres.defaultValueConverter;
export const oracleTypeConverter = oracle.typeConverter;
export const oracleDefaultValueConverter = oracle.defaultValueConverter;
export const sqlserverTypeConverter = sqlserver.typeConverter;
export const sqlserverDefaultValueConverter = sqlserver.defaultValueConverter;
