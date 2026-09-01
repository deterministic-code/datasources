import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import {
  ConsoleCommandMiddleware,
  DefaultTypeConverters,
  Repository,
  RowTypeConverter,
  WhereBuilder,
  type Cell,
  type Cells,
  type Constructable,
  type IConnection,
  type Scalar,
} from "@datasources/core";
import {
  expectedRepositoryConsoleSignatures,
  type CanonicalType,
  type RepositoryRow,
  type RepositorySpec,
  type RepositoryTypeSpec,
} from "../specs/expected-repository-console.ts";
import { consoleSignature } from "../specs/expected-console-output.ts";
import { repoRoot, specVerbose } from "./sql-spec-runner.ts";
import {
  SqliteStatementBuilder,
  type SqliteColumn,
} from "./providers/sqlite/src/sqliteStatementBuilder.ts";
import "./providers/sqlite/src/converters/SqliteTypes.ts";

export const repositorySpecFiles = (root = repoRoot()): string[] =>
  readdirSync(join(root, "specs", "repositories"))
    .filter((name) => name.endsWith(".yaml"))
    .map((name) => join(root, "specs", "repositories", name));

const defaultOf = (canonical: CanonicalType): Cell =>
  ({
    int: 0,
    uuid: "00000000-0000-0000-0000-000000000000",
    string: "",
    bool: false,
    decimal: "0",
    float: 0.5,
    datetime: new Date(0),
    binary: new Uint8Array(),
  })[canonical];

const typeOf = (type: RepositoryTypeSpec): Constructable<Cells> => {
  const defaults: Cells = Object.fromEntries(
    Object.entries(type.fields).map(([name, canonical]) => [name, defaultOf(canonical)]),
  );
  return class {
    constructor() {
      Object.assign(this, defaults);
    }
  };
};

const columnsOf = (type: RepositoryTypeSpec): SqliteColumn[] =>
  Object.entries(type.fields).map(([name, canonical]) => ({
    name,
    type: canonical,
    id: type.ids.includes(name),
  }));

const whereOf = (mapping: RowTypeConverter<Cells>, row: RepositoryRow) =>
  WhereBuilder.and(
    ...Object.entries(mapping.reverseConvert(row)).map(([name, value]) =>
      WhereBuilder.equals(name, value as Scalar),
    ),
  );

const attach = (repo: Repository<Cells>, logs: string[]) => {
  const write = (line: string) => {
    logs.push(line);
    if (specVerbose()) console.log(line);
  };
  repo.use(
    new ConsoleCommandMiddleware(write, undefined, {
      isVerbose: specVerbose() ? "true" : "false",
    }),
  );
};

const cellOf = (value: Cell): Scalar =>
  value instanceof Date
    ? value.toISOString()
    : value instanceof Uint8Array
      ? Buffer.from(value).toString("base64")
      : typeof value === "bigint"
        ? Number(value)
        : typeof value === "string" && value.includes("T") && !Number.isNaN(Date.parse(value))
          ? new Date(value).toISOString()
          : value;

const rowOf = (row: Cells): RepositoryRow =>
  Object.fromEntries(Object.entries(row).map(([key, value]) => [key, cellOf(value)]));

const numeric = (value: Scalar) =>
  typeof value === "number" || (typeof value === "string" && /^-?\d+(\.\d+)?$/.test(value));

const same = (got: Scalar, wanted: Scalar) =>
  got === wanted ||
  (numeric(got) && numeric(wanted) && Number(got) === Number(wanted));

const assertRow = (name: string, actual: Cells | undefined, expect: RepositoryRow | null) => {
  if (expect === null) {
    if (actual !== undefined) throw new Error(`${name}: expected null`);
    return;
  }
  if (actual === undefined) throw new Error(`${name}: expected a row`);
  const got = rowOf(actual);
  const wanted = rowOf(expect);
  for (const key of Object.keys(wanted))
    if (!same(got[key] as Scalar, wanted[key] as Scalar))
      throw new Error(`${name}: ${key}: expected ${JSON.stringify(wanted[key])} got ${JSON.stringify(got[key])}`);
};

const assertRows = (name: string, actual: Cells[], expect: RepositoryRow[]) => {
  if (actual.length !== expect.length)
    throw new Error(`${name}: expected ${expect.length} rows got ${actual.length}`);
  actual.forEach((row, index) => assertRow(`${name}[${index}]`, row, expect[index] ?? null));
};

export const runRepositorySpec = async (
  path: string,
  connection: IConnection,
  execute: (sql: string) => Promise<unknown>,
): Promise<void> => {
  const spec = parse(readFileSync(path, "utf8")) as RepositorySpec;
  if (specVerbose()) console.log(`\n--- ${spec.name} (${connection.provider}) ---`);
  await execute(SqliteStatementBuilder.createTable(spec.type.table, columnsOf(spec.type)).getText());
  const mapping = new RowTypeConverter(
    DefaultTypeConverters.for(connection.provider),
    Object.fromEntries(
      Object.entries(spec.type.fields).flatMap(([name, canonical]) => {
        const converter = DefaultTypeConverters.for(connection.provider)[canonical];
        return converter ? [[name, converter]] : [];
      }),
    ),
  );
  const repo = new Repository<Cells>({
    connection,
    ids: spec.type.ids,
    assignIdentityOnAdd: spec.type.assignIdentityOnAdd,
    type: typeOf(spec.type),
    table: spec.type.table,
    mapping,
  });
  const logs: string[] = [];
  attach(repo, logs);
  for (const step of spec.steps) {
    await ({
      CREATE: async () => {
        if (step.CREATE) await repo.add(step.CREATE.row);
      },
      GET: async () => {
        if (!step.GET) return;
        const row = await repo.getSingle(WhereBuilder.equals("id", step.GET.id));
        if ("expect" in step.GET) assertRow(`${spec.name} GET`, row, step.GET.expect ?? null);
      },
      QUERY: async () => {
        if (!step.QUERY) return;
        const query = step.QUERY.where ? whereOf(mapping, step.QUERY.where) : undefined;
        const rows: Cells[] = [];
        for await (const row of repo.get(query)) rows.push(row);
        if (step.QUERY.expect) assertRows(`${spec.name} QUERY`, rows, step.QUERY.expect);
      },
      GETBY: async () => {
        if (!step.GETBY) return;
        const row = await repo.getSingle(whereOf(mapping, step.GETBY.where));
        if ("expect" in step.GETBY) assertRow(`${spec.name} GETBY`, row, step.GETBY.expect ?? null);
      },
      UPDATE: async () => {
        if (step.UPDATE) await repo.update(step.UPDATE.row);
      },
      UPDATEBY: async () => {
        if (step.UPDATEBY)
          await repo.update(step.UPDATEBY.row, whereOf(mapping, step.UPDATEBY.where));
      },
      DELETE: async () => {
        if (step.DELETE) await repo.delete({ id: step.DELETE.id } as Cells);
      },
      DELETEBY: async () => {
        if (step.DELETEBY) await repo.delete({} as Cells, whereOf(mapping, step.DELETEBY.where));
      },
    }[Object.keys(step)[0] ?? ""] ??
      (() => {
        throw new Error(`${spec.name}: empty step`);
      }))();
  }
  const expected = expectedRepositoryConsoleSignatures(spec);
  const actual = logs
    .filter((line) => /\[(Start|Finish)\]/.test(line))
    .map(consoleSignature);
  let mismatch = actual.findIndex((line, i) => line !== expected[i]);
  if (mismatch === -1 && actual.length !== expected.length)
    mismatch = Math.min(actual.length, expected.length);
  if (mismatch === -1) return;
  throw new Error(
    `${spec.name}: first logging mismatch at ${mismatch}\nexpected: ${expected[mismatch] ?? "<missing>"}\nactual:   ${actual[mismatch] ?? "<missing>"}\n--- expected ---\n${expected.join("\n")}\n--- actual ---\n${actual.join("\n")}`,
  );
};

export type { Scalar };
