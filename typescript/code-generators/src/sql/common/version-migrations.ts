import { parse } from "yaml";
import type { IStatement } from "@deterministic-code/datasources";
import { CreateTableStatement } from "@deterministic-code/datasources";
import type { SeedRow } from "@deterministic-code/deterministic-generators-common/parser";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import { memoryReader } from "@deterministic-code/generators-common/deterministic-reader";
import { DeterministicParser, VersionSnapshot } from "@deterministic-code/deterministic-generators-common/parser";
import { createCasing, type PackCasing } from "./default-casing.ts";
import { SqlMapping } from "./sql-mapping.ts";
import { buildLiveTables, sqlTablesFrom } from "./sql-schema.ts";
import { renderSql, type SqlDialect } from "./sql-dialect.ts";
import { schemaDiff, type SchemaSnapshot } from "./schema-diff.ts";
import { seedSections } from "./seed-generator.ts";
import { byGroupsFromDatasource } from "./datasource-by-fields.ts";
import { procedureStatementsForTable } from "./generate-procedures.ts";
import { datasourceSettings, hasAuditColumns, type LiveTable } from "./sql-schema.ts";

const versionDiff = /^version-\d{8}T\d{6}Z-\d{3}\/diff\.yaml$/;

const defaultDatasourceYaml = `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type" || tag == "view_type"
types: []
`;

export const versionDiffPaths = (settings: Record<string, string>): string[] =>
  (settings["version.diffs"] ?? "")
    .split("\n")
    .map((path) => path.trim())
    .filter((path) => versionDiff.test(path));

const own = (obj: object, key: string) => Object.prototype.hasOwnProperty.call(obj, key);

type Diff = Parameters<typeof VersionSnapshot.combine>[0][number];

const parseDiff = (text: string, source: string): Diff => {
  const value = parse(text) as Diff;
  if (value == null || typeof value !== "object" || Array.isArray(value))
    throw new Error(`${source}: expected a mapping`);
  return value;
};

const fold = (path: string, text: string): string =>
  VersionSnapshot.combine([{ [path]: { added: true, content: parse(text) as Diff } }]).yaml(path);

const matches = (snapshot: VersionSnapshot, path: string, live: string | undefined, whenMissing: string | undefined): boolean => {
  const has = own(snapshot.documents, path);
  const text = live === undefined || live === "" ? whenMissing : live;
  if (text === undefined || text === "") return !has;
  const snapshotYaml = has ? snapshot.yaml(path) : whenMissing === undefined ? undefined : fold(path, whenMissing);
  return snapshotYaml !== undefined && snapshotYaml === fold(path, text);
};

const filesOf = (snapshot: VersionSnapshot): Record<string, string> => {
  const files: Record<string, string> = {};
  for (const path of Object.keys(snapshot.documents)) files[path] = snapshot.yaml(path);
  files["datasource.yaml"] ??= defaultDatasourceYaml;
  return files;
};

const compile = async (
  snapshot: VersionSnapshot,
  settings: Record<string, string>,
): Promise<SchemaSnapshot & { seeds: Map<string, SeedRow[]> }> => {
  const spec = await DeterministicParser(memoryReader(filesOf(snapshot))).parse(settings);
  const types = sqlTablesFrom(spec);
  const mapping = new SqlMapping(createCasing(settings), types);
  return { tables: buildLiveTables(types, mapping), mapping, seeds: spec.datasourceSeeds };
};

const emptySnapshot = (settings: Record<string, string>): Promise<SchemaSnapshot & { seeds: Map<string, SeedRow[]> }> =>
  compile(VersionSnapshot.combine([]), settings);

const pad = (n: number): string => String(n).padStart(4, "0");

const renderAll = (dialect: SqlDialect, statements: IStatement[]): string =>
  statements
    .map((statement) => renderSql(dialect, statement))
    .filter((sql) => sql !== "")
    .join("\n");

/** Replay frozen diffs, fail when live types/datasource YAML drifted, emit per-version SQL. */
export const versionMigrationFiles = async (
  dialect: SqlDialect,
  ctx: GenerateContext,
  casing: PackCasing,
  live: { typesYaml: string | undefined; datasourceYaml: string | undefined },
): Promise<{ files: { path: string; content: string }[]; count: number }> => {
  const paths = versionDiffPaths(ctx.settings);
  if (paths.length === 0) return { files: [], count: 0 };

  const diffs: Diff[] = [];
  const snapshots: VersionSnapshot[] = [];
  for (const path of paths) {
    const text = await ctx.reader.read(path);
    if (text === "") continue;
    diffs.push(parseDiff(text, path));
    snapshots.push(VersionSnapshot.combine(diffs));
  }
  if (snapshots.length === 0) return { files: [], count: 0 };

  const last = snapshots[snapshots.length - 1]!;
  if (!matches(last, "types.yaml", live.typesYaml, undefined) || !matches(last, "datasource.yaml", live.datasourceYaml, defaultDatasourceYaml)) {
    throw new Error("types.yaml or datasource.yaml differs from the frozen version diffs; run datasources version first");
  }

  const files: { path: string; content: string }[] = [];
  let previous = await emptySnapshot(ctx.settings);
  let number = 0;
  for (const [index, snapshot] of snapshots.entries()) {
    const current = await compile(snapshot, ctx.settings);
    const up = [
      ...schemaDiff(dialect, previous, current),
      ...procedureDiff(dialect, ctx.settings, previous, current),
    ];
    const down = [
      ...schemaDiff(dialect, current, previous),
      ...procedureDiff(dialect, ctx.settings, current, previous),
    ];
    if (up.length === 0 && down.length === 0) {
      previous = current;
      continue;
    }
    number += 1;
    const seeds =
      index === 0 && up.some((statement) => statement instanceof CreateTableStatement)
        ? seedSections(dialect, current.tables, current.seeds, current.mapping)
        : [];
    const upSql = [
      `-- Generated migration for ${dialect}`,
      renderAll(dialect, up),
      ...(seeds.length ? ["", "-- Seed data", ...seeds] : []),
    ]
      .filter((line) => line !== "")
      .join("\n");
    const downSql = [`-- Rollback migration for ${dialect}`, renderAll(dialect, down)].join("\n");
    files.push({
      path: `${pad(number)}_up.sql`,
      content: finalize(upSql, casing),
    });
    files.push({
      path: `${pad(number)}_down.sql`,
      content: finalize(downSql, casing),
    });
    previous = current;
  }
  return { files, count: number };
};

const fieldsOf = (table: LiveTable): string =>
  JSON.stringify(table.fields.map((field) => [field.name, field.type, field.size ?? null]));

const procedureDiff = (
  dialect: SqlDialect,
  settings: Record<string, string>,
  before: SchemaSnapshot,
  after: SchemaSnapshot,
): IStatement[] => {
  if (!datasourceSettings(settings).useStoredProcedures) return [];
  const casing = createCasing(settings);
  const previous = new Map(before.tables.map((table) => [table.name, table]));
  const groups = byGroupsFromDatasource(after.tables);
  return after.tables.flatMap((table) => {
    const prior = previous.get(table.name);
    if (prior === undefined || !hasAuditColumns(table) || fieldsOf(prior) === fieldsOf(table)) return [];
    return procedureStatementsForTable(dialect, table, after.mapping, casing, groups.get(table.name) ?? []).statements;
  });
};

const finalize = (text: string, casing: PackCasing): string => {
  const out = casing.applyKeywords(text.replace(/\n{3,}/g, "\n\n"));
  return out.endsWith("\n") ? out : `${out}\n`;
};
