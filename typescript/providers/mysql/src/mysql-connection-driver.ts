import type { IMysqlClient, MysqlCreateConnection } from "./i-mysql-driver.ts";

export class MysqlConnectionDriver {
  private client: IMysqlClient | undefined;

  constructor(
    private readonly url: string,
    private readonly createConnection: MysqlCreateConnection,
  ) {}

  async open(_url: string): Promise<void> {
    if (this.client) return;
    this.client = await this.createConnection(this.url);
    await this.client.query(
      "SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')",
    );
  }

  async close(): Promise<void> {
    await this.client?.end();
    this.client = undefined;
  }

  require(): IMysqlClient {
    if (!this.client) throw new Error("MySQL driver is not open");
    return this.client;
  }
}
