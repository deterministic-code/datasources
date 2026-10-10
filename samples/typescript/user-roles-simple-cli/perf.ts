import { BatchCommandIBulkWriteStrategy, BulkCRUDRepository, Native, StoredProcedureBulkWriteStrategy, type IBulkWriteStrategy, type IConnection } from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { migrate } from "./src/migrate.ts";

class User {
  id = 0;
  username = "";
  password = "";
  is_enabled = true;
  created = new Date();
  updated = new Uint8Array([1, 2, 3]);
}

const sample = dirname(fileURLToPath(import.meta.url));
const shared = resolve(sample, "../../user-roles-simple-cli-shared");
const provider = process.env.PROVIDER ?? "sqlite";
const strategyName = process.env.PERF_STRATEGY ?? "batch";
const rows = Number(process.env.PERF_ROWS ?? 1000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const url = process.env.DATASOURCE_URL ?? "sqlite:///:memory:";

const driver = () =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const split = (sql: string) =>
  sql
    .split(/^---\s*$/m)
    .map((part) => part.trim())
    .filter(Boolean);

const strategy = (): IBulkWriteStrategy<User> | undefined => {
  if (strategyName === "single") return undefined;
  if (strategyName === "batch") return new BatchCommandIBulkWriteStrategy<User>();
  if (strategyName === "procedure")
    return new StoredProcedureBulkWriteStrategy<User>("bulk_create_user", "bulk_update_user", "bulk_delete_user");
  throw new Error(strategyName);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  if (strategyName !== "batch") return write(items);
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const count = async (connection: IConnection, table: string) => {
  const reader = await connection.createCommand().executeReaderNative(new Native(`SELECT COUNT(*) AS n FROM ${table}`));
  await reader.next();
  const n = reader.current?.n;
  return typeof n === "number" ? n : Number(n);
};

const connection = driver().connection({ DATASOURCE_URL: url });
await connection.open();
try {
  const command = connection.createCommand();
  await migrate(command);
  if (provider === "mysql" || provider === "postgres")
    for (const sql of split(readFileSync(resolve(shared, `bulk-${provider}.sql`), "utf8")))
      await command.executeNative(new Native(sql));
  const usersRepo = new BulkCRUDRepository(connection, { type: User, entity: "users" }, strategy());
  const createdAt = new Date("2024-01-01T00:00:00.000Z");
  const users = Array.from({ length: rows }, (_, i) =>
    Object.assign(new User(), {
      ...(strategyName === "batch" ? { id: i + 1 } : {}),
      username: `user${i + 1}`,
      password: "hash",
      is_enabled: true,
      created: createdAt,
      updated: new Uint8Array([1, 2, 3]),
    }),
  );
  const createStarted = performance.now();
  await each(users, (batch) => usersRepo.createBulk(batch));
  const createMs = performance.now() - createStarted;
  if ((await count(connection, "users")) !== rows) throw new Error("create count");
  for (const user of users) user.is_enabled = false;
  const updateStarted = performance.now();
  await each(users, (batch) => usersRepo.updateBulk(batch));
  const updateMs = performance.now() - updateStarted;
  process.stdout.write(
    `${JSON.stringify({
      sample: "user-roles-simple-cli",
      language: "typescript",
      provider,
      strategy: strategyName,
      rows,
      chunk: strategyName === "batch" ? chunk : rows,
      createMs,
      updateMs,
      createPerSec: rows / (createMs / 1000),
      updatePerSec: rows / (updateMs / 1000),
      counts: { users: rows },
    })}\n`,
  );
} finally {
  await connection.close().catch(() => undefined);
}
