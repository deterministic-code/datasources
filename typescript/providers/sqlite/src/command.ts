import {
  ArrayReader,
  CommandMiddlewareHost,
  DatasourceError,
  Native,
  ReturnStatement,
  SelectStatement,
  StatementTextRenderer,
  parametersToRecord,
  Response,
  splitStatements,
  StreamingReader,
  type Cell,
  type Cells,
  type Scalar,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";
import type { SqliteStatement } from "./i-sqlite-driver.ts";
import { reverse, SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";
import { SqliteStatementRenderer } from "./SqliteStatementRenderer.ts";

const bound = (parameters?: Parameter[]): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  return Object.fromEntries(
    Object.entries(record).map(([name, value]) => [name, reverse(value)]),
  ) as Record<string, Scalar>;
};

export class SqliteCommand extends CommandMiddlewareHost implements ICommand {
  private readonly statements: StatementTextRenderer;

  constructor(readonly connection: SqliteConnection, statements?: StatementTextRenderer) {
    super();
    this.statements = statements ?? new StatementTextRenderer(t => new SqliteStatementRenderer(t));
  }

  async executeReaderNative(
    text: Native,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new ArrayReader([]), rows: [] };
      this.connection.ensureOpen();
      const statements = splitStatements(query);
      if (statements.length === 0) return { value: new ArrayReader([]), rows: [] };
      const paramsBound = bound(params);
      let index = 0;
      let iter = this.iterate(statements[0] as string, paramsBound);
      return {
        value: new StreamingReader(
          async () => {
            const step = iter.next();
            return step.done ? undefined : step.value;
          },
          async () => {
            iter.return();
            index += 1;
            const sql = statements[index];
            if (!sql) return false;
            iter = this.iterate(sql, paramsBound);
            return true;
          },
          async () => {
            iter.return();
          },
        ),
        rows: [],
      };
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(new Native(this.statements.render(statement)), parameters);
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    return statement instanceof SelectStatement || statement instanceof ReturnStatement
      ? this.firstCell(statement, parameters)
      : this.executeNative(new Native(this.statements.render(statement)), parameters);
  }

  private async firstCell(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    const reader = await this.executeReaderStatement(statement, parameters);
    try {
      if (!(await reader.next())) return new Response(null, 0, parameters);
      const row = reader.current;
      /* v8 ignore next */
      if (!row) throw new DatasourceError("Reader current must be a row");
      const values = Object.values(row);
      return new Response(values.length === 0 ? null : values[0], 1, parameters);
    } finally {
      await reader.dispose();
    }
  }

  async executeNative(text: Native, parameters?: Parameter[]): Promise<Response> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new Response(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      const info = this.connection.require().prepare(query).run(bound(params));
      return { value: new Response(null, info.changes, params), rows: [] };
    });
  }

  private iterate(sql: string, params: Record<string, Scalar>): IterableIterator<Cells> & {
    return(): IteratorResult<Cells>;
  } {
    const statement = this.connection.require().prepare(sql) as ColumnStatement;
    const columns = statement.columns?.() ?? [];
    const names = qualified(columns);
    const declared = columns.map((column) => column.type ?? null);
    const raw = names.some((name, index) => name !== columns[index]?.name)
      ? statement.raw?.(true).iterate(params)
      : undefined;
    const rows = raw ? namedRows(raw, names, declared) : closable(statement.iterate(params), columns);
    return rows[Symbol.iterator]();
  }
}

type SqliteColumn = { name: string; column: string | null; table: string | null; type?: string | null };

const booleanDeclared = (declared?: string | null): boolean =>
  declared?.toUpperCase() === "BOOLEAN";

const cell = (value: Cell | null | undefined, declared?: string | null): Cell | null =>
  value == null ? null : booleanDeclared(declared) ? SqliteRowTypeConverters.bool.convert(value) : value;

type ColumnStatement = SqliteStatement & {
  columns?: () => SqliteColumn[];
  raw?: (enabled: boolean) => {
    iterate(params?: Record<string, Scalar>): IterableIterator<Cell[]>;
  };
};

const closable = (
  iter: IterableIterator<Cells>,
  columns: SqliteColumn[],
): IterableIterator<Cells> & { return(): IteratorResult<Cells> } => {
  const stop = iter.return ? iter.return.bind(iter) : undefined;
  const rows: IterableIterator<Cells> & { return(): IteratorResult<Cells> } = {
    next: () => {
      const step = iter.next();
      if (step.done || !step.value) return step;
      const value = Object.fromEntries(
        Object.entries(step.value).map(([name, item]) => [
          name,
          cell(item, columns.find((column) => column.name === name)?.type),
        ]),
      ) as Cells;
      return { done: false, value };
    },
    return: () => {
      if (stop) stop();
      return { done: true, value: undefined };
    },
    [Symbol.iterator]() {
      return rows;
    },
  };
  return rows;
};

const namedRows = (
  iter: IterableIterator<Cell[]>,
  names: string[],
  declared: (string | null)[],
): IterableIterator<Cells> & { return(): IteratorResult<Cells> } => {
  /* v8 ignore next */
  const stop = iter.return ? iter.return.bind(iter) : undefined;
  const rows: IterableIterator<Cells> & { return(): IteratorResult<Cells> } = {
    next: () => {
      const step = iter.next();
      return step.done
        ? { done: true, value: undefined }
        : {
            done: false,
            value: Object.fromEntries(
              names.map((name, index) => [name, cell(step.value[index], declared[index])]),
            ),
          };
    },
    return: () => {
      /* v8 ignore next */
      if (stop) stop();
      return { done: true, value: undefined };
    },
    [Symbol.iterator]() {
      return rows;
    },
  };
  return rows;
};

const qualified = (columns: SqliteColumn[]): string[] => {
  const names = columns.map((column) => column.name);
  const duplicated = new Set(names.filter((name, index) => names.indexOf(name) !== index));
  return duplicated.size === 0
    ? names
    : columns.map((column, index) =>
        duplicated.has(names[index]!) && column.table && column.column
          ? `${column.table}.${column.column}`
          : names[index]!
      );
};
