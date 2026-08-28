import { IsolationLevel, type IsolationLevelName } from "@datasources/core";
import { toPostgres, withReturning } from "./bind.ts";
import type {
  IPgClient,
  IPostgresDriver,
  PgClientCtor,
  PostgresExecuteResult,
} from "./i-postgres-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : isolation === IsolationLevel.RepeatableRead
        ? "REPEATABLE READ"
        : "SERIALIZABLE";

export class PgDriver implements IPostgresDriver {
  private client: IPgClient | undefined;

  constructor(
    private readonly url: string,
    private readonly Client: PgClientCtor,
    private readonly schema?: string,
  ) {}

  async open(_url: string): Promise<void> {
    if (this.client) return;
    this.client = new this.Client({ connectionString: this.url });
    await this.client.connect();
    if (this.schema)
      await this.client.query(`SET search_path TO ${this.schema}`);
  }

  async close(): Promise<void> {
    await this.client?.end();
    this.client = undefined;
  }

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<PostgresExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0 };
    const rewritten = withReturning(sql);
    const { text, values } = toPostgres(rewritten, params);
    const result = await this.require().query(text, values);
    return {
      rowsAffected: result.rowCount ?? result.rows.length,
      result: result.rows[0]?.id,
    };
  }

  async query(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<Cells[]> {
    if (!sql.trim()) return [];
    const { text, values } = toPostgres(sql, params);
    return (await this.require().query(text, values)).rows;
  }

  async begin(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<void> {
    await this.require().query(
      `BEGIN ISOLATION LEVEL ${isoSql(isolation)}`,
    );
  }

  async commit(): Promise<void> {
    await this.require().query("COMMIT");
  }

  async rollback(): Promise<void> {
    await this.require().query("ROLLBACK");
  }

  private require(): IPgClient {
    if (!this.client) throw new Error("Postgres driver is not open");
    return this.client;
  }
}
