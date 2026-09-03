import type { IPgClient, PgClientCtor } from "./i-postgres-driver.ts";

export class PostgresConnectionDriver {
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

  require(): IPgClient {
    if (!this.client) throw new Error("Postgres driver is not open");
    return this.client;
  }
}
