import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import { parse } from "yaml";
import {
  ConsoleCommandMiddleware,
  CreateTableStatement,
  CRUDRepository,
  MetaDataRowTypeConverter,
  TransformMapper,
  Literal,
  Operator,
  Where,
  type Cell,
  type Cells,
  type Constructable,
  type IConnection,
  type Scalar,
  type TypeConverter,
} from "@deterministic-code/datasources";
import {
  expectedRepositoryConsoleSignatures,
  flattenRepositoryRow,
  physicalTableOf,
  type AffectedExpect,
  type CanonicalType,
  type RepositoryRow,
  type RepositorySpec,
  type RepositoryTypeSpec,
  type CRUDMapperSpec,
} from "../specs/expected-repository-console.ts";
import { consoleSignature } from "../specs/expected-console-output.ts";
import { tables } from "./schema-spec.ts";
import { specsDir, specVerbose } from "./sql-spec-runner.ts";

export type RepositoryDialect = {
  quote: (id: string) => string;
};

export const repositorySpecFiles = (root = specsDir()): string[] =>
  readdirSync(join(root, "repositories"))
    .filter((name) => name.endsWith(".yaml"))
    .map((name) => join(root, "repositories", name));

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

const unquoteSql = (text: string): string =>
  text.replaceAll(/`([^`]+)`/g, "$1").replaceAll(/"([^"]+)"/g, "$1");

type SpecValue = Cell | SpecRow;
type SpecRow = { [key: string]: SpecValue };

const whereOf = (mapping: MetaDataRowTypeConverter<SpecRow>, row: RepositoryRow) => {
  const parts = Object.entries(mapping.reverseConvert(row)).map(
    ([name, value]) => new Operator.Equals(name, value as Scalar),
  );
  return new Where(
    parts.length === 0
      ? new Operator.Equals(new Literal(1), new Literal(1))
      : parts.reduce((left, right) => new Operator.And(left, right)),
  );
};

const attach = (repo: CRUDRepository<SpecRow>, logs: string[]) => {
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

const crudFieldsOf = (type: RepositoryTypeSpec, quote: (id: string) => string) => {
  const mapper: CRUDMapperSpec = type.crudMapper ?? {};
  return {
    entityMapper: new TransformMapper(
      (name: string) => quote(mapper.entity ?? name),
      (name: string) => name,
    ),
  };
};

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
  execute: (statement: CreateTableStatement) => Promise<unknown>,
  dialect: RepositoryDialect,
  types: Record<string, TypeConverter<Cell, Cell>>,
): Promise<void> => {
  const spec = parse(readFileSync(path, "utf8")) as RepositorySpec;
  if (specVerbose()) console.log(`\n--- ${spec.name} (${connection.provider}) ---`);
  const rename = spec.type.crudMapper?.entity && spec.type.crudMapper.entity !== spec.type.table
    ? { [spec.type.table]: spec.type.crudMapper.entity }
    : undefined;
  for (const statement of tables(spec.schema, dialect.quote, rename))
    await execute(statement);
  const mapping = new MetaDataRowTypeConverter<SpecRow>(fieldsOf(spec.type, types));
  const mappings = spec.type.mappings ?? {};
  const flatten = (row: RepositoryRow): RepositoryRow => flattenRepositoryRow(row, mappings);
  const repo = new CRUDRepository<SpecRow>(connection, {
    ids: spec.type.ids,
    type: typeOf(spec.type),
    entity: spec.type.table,
    objectConverter: mapping,
    ...crudFieldsOf(spec.type, dialect.quote),
    readProperties: Object.keys(spec.type.fields),
  });
  const logs: string[] = [];
  attach(repo, logs);
  for (const step of spec.steps) {
    await ({
      CREATE: async () => {
        if (step.CREATE) await repo.create(flatten(step.CREATE.row));
      },
      READ: async () => {
        if (!step.READ) return;
        const row = await repo.readSingle(new Where(new Operator.Equals("id", step.READ.id)));
        if ("expect" in step.READ) assertRow(`${spec.name} READ`, row, step.READ.expect ? flatten(step.READ.expect) : null);
      },
      QUERY: async () => {
        if (!step.QUERY) return;
        const query = step.QUERY.where ? whereOf(mapping, flatten(step.QUERY.where)) : undefined;
        const rows: SpecRow[] = [];
        for await (const row of repo.read(query)) rows.push(row);
        if (step.QUERY.expect) assertRows(`${spec.name} QUERY`, rows, step.QUERY.expect.map(flatten));
      },
      READBY: async () => {
        if (!step.READBY) return;
        const row = await repo.readSingle(whereOf(mapping, flatten(step.READBY.where)));
        if ("expect" in step.READBY) assertRow(`${spec.name} READBY`, row, step.READBY.expect ? flatten(step.READBY.expect) : null);
      },
      UPDATE: async () => {
        if (!step.UPDATE) return;
        assertAffected(`${spec.name} UPDATE`, await repo.update(flatten(step.UPDATE.row)), step.UPDATE.expect);
      },
      UPDATEBY: async () => {
        if (!step.UPDATEBY) return;
        assertAffected(
          `${spec.name} UPDATEBY`,
          await repo.update(flatten(step.UPDATEBY.row), whereOf(mapping, flatten(step.UPDATEBY.where))),
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
  const expected = expectedRepositoryConsoleSignatures(spec, connection.provider);
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
