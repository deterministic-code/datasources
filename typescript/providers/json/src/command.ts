import {
  ArrayReader,
  CommandMiddlewareHost,
  DeleteStatement,
  InsertStatement,
  InsertedIdStatement,
  UpdateStatement,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type IWhere,
  type Response,
  type Parameter,
} from "@datasources/core";
import type { JsonConnection } from "./connection.ts";
import {
  asRecord,
  matches,
  parametersToObject,
  parseFilter,
} from "./filter.ts";

export class JsonCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: JsonConnection) {
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
        .filter((row) => matches(asRecord(row), filter));
      const cells = rows.map(asRecord) as Cells[];
      return { value: new ArrayReader(cells), rows: cells };
    });
  }

  executeReaderStatement = (
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> =>
    statement instanceof InsertedIdStatement
      ? this.run(statement.getText(), parameters, async () => {
          this.connection.ensureOpen();
          return { value: new ArrayReader([]), rows: [] };
        })
      : this.executeReader(filterText(statement), parameters);

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
          this.connection.getCollection().push(obj);
          await this.connection.persist();
          return { value: { result: obj, rowsAffected: 1, parameters: params }, rows: [] };
        }
        if (op === "update") {
          const whereText =
            typeof obj.$where === "string" ? obj.$where : undefined;
          const next = { ...obj };
          delete next.$where;
          const filter = whereText ? parseFilter(whereText) : next;
          return { value: { ...(await this.applyUpdate(next, filter)), parameters: params }, rows: [] };
        }
        if (op === "delete") {
          const filter = obj;
          return { value: { ...(await this.applyDelete(filter)), parameters: params }, rows: [] };
        }
        throw new Error(`Unsupported JSON command: ${query}`);
    });
  }

  private async applyUpdate(
    obj: Record<string, unknown>,
    filter: Record<string, unknown>,
  ): Promise<Response> {
    const collection = this.connection.getCollection();
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(asRecord(collection[i]), filter)) {
        collection[i] = { ...asRecord(collection[i]), ...obj };
        rowsAffected += 1;
      }
    }
    await this.connection.persist();
    return { result: obj, rowsAffected };
  }

  private async applyDelete(
    filter: Record<string, unknown>,
  ): Promise<Response> {
    const collection = this.connection.getCollection();
    const remaining = collection.filter(
      (row) => !matches(asRecord(row), filter),
    );
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    await this.connection.persist();
    return { result: undefined, rowsAffected };
  }
}

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
