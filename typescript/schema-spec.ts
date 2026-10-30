import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";
import {
  CheckConstraint,
  ColumnDefault,
  CreateTableStatement,
  execute,
  ForeignKey,
  NotNull,
  PrimaryKey,
  UniqueConstraint,
  type Column,
  type IConnection,
  type ITableConstraint,
} from "@deterministic-code/datasources";

type ColumnSpec = {
  name: string;
  type: string;
  primaryKey?: boolean;
  notNull?: boolean;
  default?: string;
};

type ConstraintSpec = {
  kind: string;
  name: string;
  columns?: string[];
  references?: string;
  referencesColumns?: string[];
  expression?: string;
};

type TableSpec = {
  name: string;
  use?: string[];
  columns?: ColumnSpec[];
  constraints?: ConstraintSpec[];
};

type CatalogFile = {
  fragments: Record<string, ColumnSpec[]>;
  schemas: Record<string, TableSpec[]>;
};

export const specsDir = (from = fileURLToPath(import.meta.url)): string => {
  for (let dir = dirname(from); ; dir = dirname(dir)) {
    if (existsSync(join(dir, "specs", "command-crud.yaml"))) return join(dir, "specs");
    if (dir === dirname(dir)) throw new Error("specs not found");
  }
};

let cache: CatalogFile | undefined;

const file = (): CatalogFile =>
  (cache ??= parse(readFileSync(join(specsDir(), "schemas.yaml"), "utf8")) as CatalogFile);

const physical = (logical: string, rename?: Readonly<Record<string, string>>): string =>
  rename?.[logical] ?? logical;

const statementOf = (
  table: TableSpec,
  quote: (id: string) => string,
  rename?: Readonly<Record<string, string>>,
): CreateTableStatement => {
  const specs = [
    ...(table.use ?? []).flatMap((fragment) => file().fragments[fragment] ?? (() => { throw new Error(`unknown fragment ${fragment}`); })()),
    ...(table.columns ?? []),
  ];
  const keys = specs.filter((column) => column.primaryKey).map((column) => column.name);
  const constraints: ITableConstraint[] = [
    ...(keys.length > 0 ? [new PrimaryKey(`pk_${physical(table.name, rename)}`, keys.map(quote))] : []),
    ...(table.constraints ?? []).map((constraint) => {
      const columns = (constraint.columns ?? []).map(quote);
      return ({
        primaryKey: () => new PrimaryKey(constraint.name, columns),
        unique: () => new UniqueConstraint(constraint.name, columns),
        check: () => new CheckConstraint(constraint.name, constraint.expression ?? (() => { throw new Error(`${constraint.name}: empty check`); })()),
        foreignKey: () => new ForeignKey(
          constraint.name,
          columns,
          quote(physical(constraint.references ?? (() => { throw new Error(`${constraint.name}: empty references`); })(), rename)),
          (constraint.referencesColumns ?? []).map(quote),
        ),
      }[constraint.kind] ?? (() => { throw new Error(`${constraint.name}: empty constraint`); }))();
    }),
  ];
  return new CreateTableStatement(
    quote(physical(table.name, rename)),
    specs.map((column): Column => ({
      name: quote(column.name),
      type: column.type,
      ...(column.default === undefined ? {} : { default: new ColumnDefault(column.default) }),
      ...(column.notNull ? { notNull: new NotNull() } : {}),
      ...(column.primaryKey && column.type === "int" ? { identity: true } : {}),
    })),
    constraints,
  );
};

export const tables = (
  name: string,
  quote: (id: string) => string,
  rename?: Readonly<Record<string, string>>,
): CreateTableStatement[] => {
  const schemas = file().schemas[name];
  if (!schemas) throw new Error(`unknown schema ${name}`);
  return schemas.map((table) => statementOf(table, quote, rename));
};

export const applySchema = async (connection: IConnection, name: string): Promise<void> => {
  const command = connection.createCommand();
  for (const statement of tables(name, (id) => id))
    await execute(command, statement);
};
