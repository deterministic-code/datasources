import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import {
  ConsoleCommandMiddleware,
  Repository,
  TypeConverter,
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

const whereOf = (row: RepositoryRow) =>
  WhereBuilder.and(
    ...Object.entries(row).map(([name, value]) => WhereBuilder.equals(name, value)),
  );

const attach = (repo: Repository<Cells>, logs: string[]) => {
  const write = (line: string) => {
    logs.push(line);
    if (specVerbose()) console.log(line);
  };
  repo.commands.get.use(new ConsoleCommandMiddleware(write));
  repo.commands.add.use(new ConsoleCommandMiddleware(write));
  repo.commands.update.use(new ConsoleCommandMiddleware(write));
  repo.commands.delete.use(new ConsoleCommandMiddleware(write));
};

export const runRepositorySpec = async (
  path: string,
  connection: IConnection,
  execute: (sql: string) => Promise<unknown>,
): Promise<void> => {
  const spec = parse(readFileSync(path, "utf8")) as RepositorySpec;
  if (specVerbose()) console.log(`\n--- ${spec.name} (${connection.provider}) ---`);
  await execute(SqliteStatementBuilder.createTable(spec.type.table, columnsOf(spec.type)).getText());
  const repo = new Repository<Cells>({
    connection,
    ids: spec.type.ids,
    assignIdentityOnAdd: spec.type.assignIdentityOnAdd,
    type: typeOf(spec.type),
    mapping: new TypeConverter(spec.type.table, typeOf(spec.type)),
  });
  const logs: string[] = [];
  attach(repo, logs);
  for (const step of spec.steps) {
    if (step.CREATE) await repo.add(step.CREATE.row);
    else if (step.READ) await repo.getSingle(WhereBuilder.equals("id", step.READ.id));
    else if (step.QUERY) {
      const query = step.QUERY.where ? whereOf(step.QUERY.where) : undefined;
      for await (const _row of repo.get(query)) {
        /* drain */
      }
    } else if (step.READY) await repo.getSingle(whereOf(step.READY.where));
    else if (step.UPDATE) await repo.update(step.UPDATE.row);
    else if (step.UPDATEBY) await repo.update(step.UPDATEBY.row, whereOf(step.UPDATEBY.where));
    else if (step.DELETE) await repo.delete({ id: step.DELETE.id } as Cells);
    else if (step.DELETEBY) await repo.delete({} as Cells, whereOf(step.DELETEBY.where));
    else throw new Error(`${spec.name}: empty step`);
  }
  const expected = expectedRepositoryConsoleSignatures(spec);
  const actual = logs.map(consoleSignature);
  let mismatch = actual.findIndex((line, i) => line !== expected[i]);
  if (mismatch === -1 && actual.length !== expected.length)
    mismatch = Math.min(actual.length, expected.length);
  if (mismatch === -1) return;
  throw new Error(
    `${spec.name}: first logging mismatch at ${mismatch}\nexpected: ${expected[mismatch] ?? "<missing>"}\nactual:   ${actual[mismatch] ?? "<missing>"}\n--- expected ---\n${expected.join("\n")}\n--- actual ---\n${actual.join("\n")}`,
  );
};

export type { Scalar };