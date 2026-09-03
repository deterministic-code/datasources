import { beforeAll, describe, it } from "vitest";
import { ensureCompose } from "../../ensure-compose.ts";
import {
  repositorySpecFiles,
  runRepositorySpec,
} from "../../repository-spec-runner.ts";
import {
  loadPgDriver,
  PostgresCommand,
  PostgresConnection,
  PostgresStatementBuilder,
} from "../src/index.ts";
import type { PgModule } from "../src/i-postgres-driver.ts";

const url = (process.env.POSTGRES_URL ??=
  "postgres://ds:ds@127.0.0.1:5432/datasources");

beforeAll(() => ensureCompose("postgres"), 120_000);

const isolate = async (base: string) => {
  const pg = (await import("pg")) as PgModule;
  const Client = pg.default?.Client ?? pg.Client;
  const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = new Client({ connectionString: base });
  await admin.connect();
  await admin.query(`CREATE SCHEMA ${schema}`);
  await admin.end();
  return {
    schema,
    cleanup: async () => {
      const drop = new Client({ connectionString: base });
      await drop.connect();
      await drop.query(`DROP SCHEMA ${schema} CASCADE`);
      await drop.end();
    },
  };
};

describe("repository specs", () => {
  for (const file of repositorySpecFiles()) {
    it(file.split("/").pop()!, async () => {
      const isolated = await isolate(url);
      try {
        const connection = new PostgresConnection({
          url,
          schema: isolated.schema,
          driver: await loadPgDriver(url, undefined, isolated.schema),
        });
        await connection.open();
        const command = new PostgresCommand(connection);
        await runRepositorySpec(
          file,
          connection,
          (sql) => command.execute(sql),
          PostgresStatementBuilder,
        );
      } finally {
        await isolated.cleanup();
      }
    });
  }
});
