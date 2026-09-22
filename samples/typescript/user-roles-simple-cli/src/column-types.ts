import { MysqlColumnType } from "@datasources/mysql";
import { PostgresColumnType } from "@datasources/postgres";
import { SqliteColumnType } from "@datasources/sqlite";

export const columnTypes = (provider: string): ((canonical: string) => string) =>
  ({
    sqlite: SqliteColumnType.map,
    mysql: MysqlColumnType.map,
    postgres: PostgresColumnType.map,
  }[provider] ??
    (() => {
      throw new Error(`unknown provider ${provider}`);
    })());
