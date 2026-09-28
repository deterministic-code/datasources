import type { IStatement } from "./IStatement.ts";

export class InsertedIdStatement implements IStatement {
  constructor(
    readonly table: string,
    readonly column: string,
  ) {}

  getText = (): string => `${this.table}.${this.column}`;

  static select = (provider: string, column: string): string => {
    const sql: Record<string, string> = {
      sqlite: `SELECT last_insert_rowid() AS ${column}`,
      mysql: `SELECT LAST_INSERT_ID() AS ${column}`,
      postgres: `SELECT lastval() AS ${column}`,
    };
    return sql[provider] ?? "";
  };
}
