import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import {
  ConsoleCommandMiddleware,
  DefaultTypeConverters,
  DeleteStatement,
  InsertStatement,
  Query,
  Repository,
  RowTypeConverter,
  StatementMapper,
  TransformMapper,
  UpdateStatement,
  WhereBuilder,
  type Cell,
  type Cells,
  type Constructable,
  type IConnection,
  type Scalar,
  type TypeConverter,
} from "@datasources/core";
import {
  expectedRepositoryConsoleSignatures,
  physicalTableOf,
  type AffectedExpect,
  type CanonicalType,
  type RepositoryMappings,
  type RepositoryRow,
  type RepositorySpec,
  type RepositoryTypeSpec,
  type StatementMapperSpec,
} from "../specs/expected-repository-console.ts";
import { consoleSignature } from "../specs/expected-console-output.ts";
import { repoRoot, specVerbose } from "./sql-spec-runner.ts";

export type RepositoryColumn = { name: string; type: string; id?: boolean };

export type RepositoryDialect = {
  createTable: (table: string, columns: RepositoryColumn[]) => { getText: () => string };
  quote: (id: string) => string;
};

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

const columnsOf = (type: RepositoryTypeSpec): RepositoryColumn[] =>
  Object.entries(type.fields).map(([name, canonical]) => ({
    name,
    type: canonical,
    id: type.ids.includes(name),
  }));

const unquoteSql = (text: string): string =>
  text.replaceAll(/`([^`]+)`/g, "$1").replaceAll(/"([^"]+)"/g, "$1");

type SpecValue = Cell | SpecRow;
type SpecRow = { [key: string]: SpecValue };

const whereOf = (mapping: RowTypeConverter<SpecRow>, row: RepositoryRow) =>
  WhereBuilder.and(
    ...Object.entries(mapping.reverseConvert(row)).map(([name, value]) =>
      WhereBuilder.equals(name, value as Scalar),
    ),
  );

const attach = (repo: Repository<SpecRow>, logs: string[]) => {
  const write = (line: string) => {
    logs.push(line);
    if (specVerbose()) console.log(line);
  };
  repo.addMiddleware(
    new ConsoleCommandMiddleware(write, undefined, {
      isVerbose: specVerbose() ? "true" : "false",
    }),
  );
};

const isRow = (value: SpecValue | RepositoryRow | Scalar | undefined): value is SpecRow =>
  value !== null &&
  value !== undefined &&
  typeof value === "object" &&
  !(value instanceof Date) &&
  !(value instanceof Uint8Array) &&
  !Array.isArray(value);

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

const rowOf = (row: SpecRow): RepositoryRow =>
  Object.fromEntries(
    Object.entries(row).map(([key, value]) => [
      key,
      isRow(value) ? rowOf(value) : cellOf(value),
    ]),
  );

const fieldsOf = (
  spec: RepositoryTypeSpec,
  catalog: Record<string, TypeConverter<Cell, Cell>>,
): Record<string, TypeConverter<Cell, Cell>> =>
  Object.fromEntries(
    [
      ...Object.entries(spec.fields),
      ...Object.entries(spec.converters ?? {}),
    ].flatMap(([name, canonical]) => {
      const converter = catalog[canonical];
      return converter ? [[name, converter]] : [];
    }),
  );

const suffix = <T extends { getText: () => string }>(
  rewrite: (text: string, value: T) => T,
  value: string | undefined,
) =>
  value
    ? new TransformMapper<T, T>((built) => rewrite(`${built.getText()}${value}`, built), (built) => built)
    : undefined;

const statementMapperOf = (
  type: RepositoryTypeSpec,
  rows: TransformMapper<SpecRow, SpecRow>,
  quote: (id: string) => string,
) => {
  const mapper: StatementMapperSpec = type.statementMapper ?? {};
  return new StatementMapper<SpecRow>({
    entity: new TransformMapper(
      (name) => quote(mapper.entity ?? name),
      (name) => name,
    ),
    rows,
    query: suffix((text, query) => new Query(text, query.where), mapper.querySuffix),
    insert: suffix((text) => new InsertStatement(text), mapper.writeSuffix),
    update: suffix((text, update) => new UpdateStatement(text, update.where), mapper.writeSuffix),
    delete: suffix((text, remove) => new DeleteStatement(text, remove.where), mapper.writeSuffix),
  });
};

const mappingsMapper = (mappings: RepositoryMappings = {}) =>
  new TransformMapper<SpecRow, SpecRow>(
    (row) => {
      const dest: SpecRow = { ...row };
      for (const [nested, cols] of Object.entries(mappings)) {
        dest[nested] = Object.fromEntries(
          Object.entries(cols).map(([field, col]) => [field, dest[col]]),
        );
        for (const col of Object.values(cols)) delete dest[col];
      }
      return dest;
    },
    (row) => {
      const dest: SpecRow = { ...row };
      for (const [nested, cols] of Object.entries(mappings)) {
        const value = dest[nested];
        if (isRow(value))
          for (const [field, col] of Object.entries(cols)) dest[col] = value[field];
        delete dest[nested];
      }
      return dest;
    },
  );

const numeric = (value: Scalar) =>
  typeof value === "number" || (typeof value === "string" && /^-?\d+(\.\d+)?$/.test(value));

const same = (got: Scalar, wanted: Scalar) =>
  got === wanted ||
  (numeric(got) && numeric(wanted) && Number(got) === Number(wanted));

const assertRow = (name: string, actual: SpecRow | undefined, expect: RepositoryRow | null) => {
  if (expect === null) {
    if (actual !== undefined) throw new Error(`${name}: expected null`);
    return;
  }
  if (actual === undefined) throw new Error(`${name}: expected a row`);
  const got = rowOf(actual);
  const wanted = rowOf(expect);
  for (const key of Object.keys(wanted)) {
    const gotValue = got[key];
    const wantedValue = wanted[key];
    if (isRow(gotValue) && isRow(wantedValue)) {
      assertRow(`${name}.${key}`, gotValue, wantedValue);
      continue;
    }
    if (!same(gotValue as Scalar, wantedValue as Scalar))
      throw new Error(`${name}: ${key}: expected ${JSON.stringify(wantedValue)} got ${JSON.stringify(gotValue)}`);
  }
};

const assertRows = (name: string, actual: SpecRow[], expect: RepositoryRow[]) => {
  if (actual.length !== expect.length)
    throw new Error(`${name}: expected ${expect.length} rows got ${actual.length}`);
  actual.forEach((row, index) => assertRow(`${name}[${index}]`, row, expect[index] ?? null));
};

const assertAffected = (name: string, actual: { rowsAffected: number }, expect?: AffectedExpect) => {
  if (!expect) return;
  if (actual.rowsAffected !== expect.rowsAffected)
    throw new Error(`${name}: expected ${expect.rowsAffected} rows affected got ${actual.rowsAffected}`);
};

export const runRepositorySpec = async (
  path: string,
  connection: IConnection,
  execute: (sql: string) => Promise<unknown>,
  dialect: RepositoryDialect,
): Promise<void> => {
  const spec = parse(readFileSync(path, "utf8")) as RepositorySpec;
  if (specVerbose()) console.log(`\n--- ${spec.name} (${connection.provider}) ---`);
  await execute(dialect.createTable(physicalTableOf(spec.type), columnsOf(spec.type)).getText());
  const catalog = DefaultTypeConverters.for(connection.provider);
  const mapping = new RowTypeConverter<SpecRow>(catalog, fieldsOf(spec.type, catalog));
  const repo = new Repository<SpecRow>({
    connection,
    ids: spec.type.ids,
    assignIdentityOnCreate: spec.type.assignIdentityOnAdd,
    type: typeOf(spec.type),
    entity: spec.type.table,
    mapping,
    statementMapper: statementMapperOf(spec.type, mappingsMapper(spec.type.mappings), dialect.quote),
    columns: Object.keys(spec.type.fields),
  });
  const logs: string[] = [];
  attach(repo, logs);
  for (const step of spec.steps) {
    await ({
      CREATE: async () => {
        if (step.CREATE) await repo.create(step.CREATE.row);
      },
      GET: async () => {
        if (!step.GET) return;
        const row = await repo.getSingle(WhereBuilder.equals("id", step.GET.id));
        if ("expect" in step.GET) assertRow(`${spec.name} GET`, row, step.GET.expect ?? null);
      },
      QUERY: async () => {
        if (!step.QUERY) return;
        const query = step.QUERY.where ? whereOf(mapping, step.QUERY.where) : undefined;
        const rows: SpecRow[] = [];
        for await (const row of repo.get(query)) rows.push(row);
        if (step.QUERY.expect) assertRows(`${spec.name} QUERY`, rows, step.QUERY.expect);
      },
      GETBY: async () => {
        if (!step.GETBY) return;
        const row = await repo.getSingle(whereOf(mapping, step.GETBY.where));
        if ("expect" in step.GETBY) assertRow(`${spec.name} GETBY`, row, step.GETBY.expect ?? null);
      },
      UPDATE: async () => {
        if (!step.UPDATE) return;
        assertAffected(`${spec.name} UPDATE`, await repo.update(step.UPDATE.row), step.UPDATE.expect);
      },
      UPDATEBY: async () => {
        if (!step.UPDATEBY) return;
        assertAffected(
          `${spec.name} UPDATEBY`,
          await repo.update(step.UPDATEBY.row, whereOf(mapping, step.UPDATEBY.where)),
          step.UPDATEBY.expect,
        );
      },
      DELETE: async () => {
        if (!step.DELETE) return;
        assertAffected(
          `${spec.name} DELETE`,
          await repo.delete({ id: step.DELETE.id }),
          step.DELETE.expect,
        );
      },
      DELETEBY: async () => {
        if (!step.DELETEBY) return;
        assertAffected(
          `${spec.name} DELETEBY`,
          await repo.delete({}, whereOf(mapping, step.DELETEBY.where)),
          step.DELETEBY.expect,
        );
      },
    }[Object.keys(step)[0] ?? ""] ??
      (() => {
        throw new Error(`${spec.name}: empty step`);
      }))();
  }
  const expected = expectedRepositoryConsoleSignatures(spec);
  const actual = logs
    .filter((line) => /\[(Start|Finish)\]/.test(line))
    .map((line) => unquoteSql(consoleSignature(line)));
  let mismatch = actual.findIndex((line, i) => line !== expected[i]);
  if (mismatch === -1 && actual.length !== expected.length)
    mismatch = Math.min(actual.length, expected.length);
  if (mismatch === -1) return;
  throw new Error(
    `${spec.name}: first logging mismatch at ${mismatch}\nexpected: ${expected[mismatch] ?? "<missing>"}\nactual:   ${actual[mismatch] ?? "<missing>"}\n--- expected ---\n${expected.join("\n")}\n--- actual ---\n${actual.join("\n")}`,
  );
};

export type { Scalar };
