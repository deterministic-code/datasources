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
    if (!clause) return this.executeReader(filterText(statement), parameters);
    return this.run(statement.getText(), parameters, async () => {
      this.connection.ensureOpen();
      const evaluator = new StatementEvaluator();
      const rows = this.connection
        .getCollection()
        .filter((row) => evaluator.evaluate(clause, row as Record<string, Scalar>) === true);
      return { value: new ArrayReader(rows as Cells[]), rows: rows as Cells[] };
    });
  };

  executeStatement = (
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<Response> =>
    statement instanceof InsertStatement
      ? this.execute("insert", parameters)
      : statement instanceof UpdateStatement
        ? this.execute("update", withWhere(parameters, statement.where))
        : statement instanceof DeleteStatement
          ? this.execute("delete", withWhere(parameters, statement.where))
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
          return { value: { ...this.applyUpdate(next, filter), parameters: params }, rows: [] };
        }
        if (op === "delete") {
          const filter =
            Object.keys(obj).length === 0 ? {} : identityFilter(obj);
          return { value: { ...this.applyDelete(filter), parameters: params }, rows: [] };
        }
        throw new Error(`Unsupported Memory command: ${query}`);
    });
  }

  private applyUpdate(
    obj: MemoryRecord,
    filter: MemoryRecord,
  ): Response {
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(collection[i], filter)) {
        collection[i] = { ...collection[i], ...obj };
        rowsAffected += 1;
      }
    }
    return { result: obj, rowsAffected };
  }

  private applyDelete(filter: MemoryRecord): Response {
    const collection = this.connection.getCollection();
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

const withWhere = (
  parameters: Parameter[] | undefined,
  where?: IWhere,
): Parameter[] | undefined =>
  looksJson(where?.getText())
    ? [...(parameters ?? []), { name: "$where", value: where!.getText() }]
    : parameters;
