import {
  ArrayReader,
  CommandMiddlewareHost,
  DeleteStatement,
  identityFilter,
  InsertStatement,
  StatementEvaluator,
  UpdateStatement,
  Where,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type IWhere,
  type Response,
  type Parameter,
  type Scalar,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";
import type { MemoryRecord } from "./store.ts";
import { matches, parametersToObject, parseFilter } from "./filter.ts";

export class MemoryCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: MemoryConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const filter = {
        ...parseFilter(query),
        ...parametersToObject(params),
      };
      const rows = this.connection
        .getCollection()
        .filter((row) => matches(row, filter));
      return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
    });
  }

  executeReaderStatement = (
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> => {
    const clause = "where" in statement && statement.where instanceof Where ? statement.where : undefined;
    if (clause) {
      return this.run(statement.getText(), parameters, async () => {
        this.connection.ensureOpen();
        const evaluator = new StatementEvaluator();
        const rows = this.connection
          .rows(tableOf(statement), false)
          .filter((row) => evaluator.evaluate(clause, row as Record<string, Scalar>) === true);
        return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
      });
    }
    if ("where" in statement && !looksJson(statement.where?.getText()) && !looksJson(statement.getText())) {
      return this.run(statement.getText(), parameters, async () => {
        this.connection.ensureOpen();
        const rows = this.connection.rows(tableOf(statement), false);
        return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
      });
    }
    return this.executeReader(filterText(statement), parameters);
  };

  executeStatement = (
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<Response> =>
    statement instanceof InsertStatement
      ? this.write("insert", parameters, tableOf(statement), true)
      : statement instanceof UpdateStatement
        ? this.write("update", parameters, tableOf(statement), false, statement.where)
        : statement instanceof DeleteStatement
          ? this.write("delete", parameters, tableOf(statement), false, statement.where)
          : this.execute(statement.getText(), parameters);

  async execute(
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
        this.connection.ensureOpen();
        const op = query.trim().toLowerCase();
        const obj = parametersToObject(params);
        if (op === "insert") {
          const row = { ...obj };
          const id = insertedId(row, this.connection.getCollection());
          row.id = id;
          this.connection.getCollection().push(row);
          return { value: { result: id, rowsAffected: 1, parameters: params }, rows: [] };
        }
        if (op === "update") {
          const whereText =
            typeof obj.$where === "string" ? obj.$where : undefined;
          const next = { ...obj };
          delete next.$where;
          const filter = whereText ? parseFilter(whereText) : identityFilter(next);
          return { value: { ...this.applyUpdate(this.connection.getCollection(), next, filter), parameters: params }, rows: [] };
        }
        if (op === "delete") {
          const filter =
            Object.keys(obj).length === 0 ? {} : identityFilter(obj);
          return { value: { ...this.applyDelete(this.connection.getCollection(), filter), parameters: params }, rows: [] };
        }
        throw new Error(`Unsupported Memory command: ${query}`);
    });
  }

  private write = (
    op: string,
    parameters: Parameter[] | undefined,
    table: string | undefined,
    create: boolean,
    where?: IWhere,
  ): Promise<Response> =>
    this.run(op, parameters, async (_query, params) => {
      this.connection.ensureOpen();
      const rows = this.connection.rows(table, create);
      const obj = parametersToObject(params);
      if (op === "insert") {
        const row = { ...obj };
        const id = insertedId(row, rows);
        row.id = id;
        rows.push(row);
        return { value: { result: id, rowsAffected: 1, parameters: params }, rows: [] };
      }
      if (op === "update") {
        return { value: { ...this.applyChanged(rows, obj, where, params), parameters: params }, rows: [] };
      }
      return { value: { ...this.removeMatched(rows, obj, where), parameters: params }, rows: [] };
    });

  private applyChanged = (
    rows: MemoryRecord[],
    obj: MemoryRecord,
    where: IWhere | undefined,
    parameters: Parameter[] | undefined,
  ): Response => {
    if (where instanceof Where) {
      const evaluator = new StatementEvaluator();
      const names = setNames(parameters, where);
      let rowsAffected = 0;
      for (const row of rows) {
        if (evaluator.evaluate(where, row as Record<string, Scalar>) !== true) continue;
        for (const name of names) if (name in obj) row[name] = obj[name];
        rowsAffected += 1;
      }
      return { result: undefined, rowsAffected };
    }
    const next = { ...obj };
    delete next.$where;
    const filter = looksJson(where?.getText()) ? parseFilter(where!.getText()) : identityFilter(next);
    return this.applyUpdate(rows, next, filter);
  };

  private removeMatched = (
    rows: MemoryRecord[],
    obj: MemoryRecord,
    where: IWhere | undefined,
  ): Response => {
    if (where instanceof Where) {
      const evaluator = new StatementEvaluator();
      const remaining = rows.filter(
        (row) => evaluator.evaluate(where, row as Record<string, Scalar>) !== true,
      );
      const rowsAffected = rows.length - remaining.length;
      rows.length = 0;
      rows.push(...remaining);
      return { result: undefined, rowsAffected };
    }
    const filter = looksJson(where?.getText())
      ? parseFilter(where!.getText())
      : Object.keys(obj).length === 0
        ? {}
        : identityFilter(obj);
    return this.applyDelete(rows, filter);
  };

  private applyUpdate(
    collection: MemoryRecord[],
    obj: MemoryRecord,
    filter: MemoryRecord,
  ): Response {
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(collection[i], filter)) {
        collection[i] = { ...collection[i], ...obj };
        rowsAffected += 1;
      }
    }
    return { result: obj, rowsAffected };
  }

  private applyDelete(collection: MemoryRecord[], filter: MemoryRecord): Response {
    const remaining = collection.filter((row) => !matches(row, filter));
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    return { result: undefined, rowsAffected };
  }
}

const insertedId = (row: MemoryRecord, rows: MemoryRecord[]): number =>
  typeof row.id === "number"
    ? row.id
    : rows.reduce((max, item) => (typeof item.id === "number" && item.id > max ? item.id : max), 0) + 1;

const looksJson = (text?: string): boolean => !!text?.trimStart().startsWith("{");

const filterText = (statement: IStatement): string => {
  const where = "where" in statement ? (statement as { where?: IWhere }).where?.getText() : undefined;
  if (looksJson(where)) return where!;
  const text = statement.getText();
  return looksJson(text) || !text.trim() ? text : "";
};

const tableOf = (statement: IStatement): string | undefined => {
  if ("table" in statement && typeof statement.table === "string" && statement.table.length > 0)
    return statement.table;
  return tableIn(statement.getText());
};

const tableIn = (text: string): string | undefined => {
  const sql = text.trimStart();
  const lower = sql.toLowerCase();
  const skip = lower.startsWith("insert into ")
    ? "insert into ".length
    : lower.startsWith("delete from ")
      ? "delete from ".length
      : lower.startsWith("update ")
        ? "update ".length
        : lower.indexOf(" from ") >= 0
          ? lower.indexOf(" from ") + " from ".length
          : -1;
  if (skip < 0 || skip >= sql.length) return undefined;
  const rest = sql.slice(skip);
  const end = rest.search(/[\s(]/);
  const name = end < 0 ? rest : rest.slice(0, end);
  return name.length > 0 ? name : undefined;
};

const setNames = (parameters: Parameter[] | undefined, where: IWhere): string[] => {
  const bound = new Set((where.parameters ?? []).map((parameter) => parameter.name));
  return (parameters ?? [])
    .map((parameter) => parameter.name)
    .filter((name) => !bound.has(name) && !name.endsWith("__where"));
};
