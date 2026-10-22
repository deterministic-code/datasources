import { BatchCommandIBulkWriteStrategy, BulkCRUDRepository, Native, StoredProcedureBulkWriteStrategy, type IBulkWriteStrategy, type IConnection } from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { users as seedUsers } from "../../../scripts/perf-data.mjs";
import { migrateUp } from "../../../scripts/migraters.mjs";

type SeedUser = {
  id: number;
  username: string;
  password: string;
  is_enabled: boolean;
  created: string;
  updated: number[];
};

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
const url = process.env.DATASOURCE_URL ?? `sqlite:///${resolve(sample, "perf.sqlite")}`;

const driver = () =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const batched = strategyName === "batch" || (strategyName === "procedure" && provider === "sqlite");

const strategy = (): IBulkWriteStrategy<User> | undefined => {
  if (strategyName === "single") return undefined;
  if (batched) return new BatchCommandIBulkWriteStrategy<User>();
  if (strategyName === "procedure")
    return new StoredProcedureBulkWriteStrategy<User>("bulk_create_user", "bulk_update_user", "bulk_delete_user");
  throw new Error(strategyName);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  if (!batched) return write(items);
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const count = async (connection: IConnection, table: string) => {
  const reader = await connection.createCommand().executeReaderNative(new Native(`SELECT COUNT(*) AS n FROM ${table}`));
  await reader.next();
  const n = reader.current?.n;
  return typeof n === "number" ? n : Number(n);
};

const sqlRoot = mkdtempSync(join(tmpdir(), "user-roles-perf-sql-"));
execFileSync(
  process.execPath,
  ["--import", "tsx", resolve(sample, "../../../cli/src/main.ts"), "migrations", "create", "--input", resolve(shared, "deterministic"), "--migrations-root", join(sqlRoot, "sql")],
  { encoding: "utf8", cwd: resolve(sample, "../../../cli") },
);
migrateUp({ provider, connection: url, migrationsRoot: join(sqlRoot, "sql") });
const connection = driver().connection({ DATASOURCE_URL: url });
await connection.open();
try {
  const usersRepo = new BulkCRUDRepository(connection, { type: User, entity: "users" }, strategy());
  const seed = (process.env.PERF_DATA
    ? JSON.parse(readFileSync(process.env.PERF_DATA, "utf8"))
    : seedUsers(rows)) as SeedUser[];
  const users = seed.map((row) =>
    Object.assign(new User(), {
      ...row,
      created: new Date(row.created),
      updated: Uint8Array.from(row.updated),
    }),
  );
  const createStarted = performance.now();
  await each(users, (batch) => usersRepo.createBulk(batch));
  const createMs = performance.now() - createStarted;
  if ((await count(connection, "users")) !== users.length) throw new Error("create count");
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
      rows: users.length,
      chunk: batched ? Math.min(chunk, users.length) : users.length,
      createMs,
      updateMs,
      createPerSec: users.length / (createMs / 1000),
      updatePerSec: users.length / (updateMs / 1000),
      counts: { users: users.length },
    })}\n`,
  );
} finally {
  await connection.close().catch(() => undefined);
}
