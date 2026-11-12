import {
  DatasourceError,
  ArrayExpression,
  DefaultStatementTextRenderer,
  DeleteStatement,
  ExpressionEvaluator,
  Identifier,
  InsertStatement,
  Literal,
  Operator,
  SelectStatement,
  StatementParser,
  UpdateStatement,
  Where,
  WhereText,
  Response,
  type IExpression,
  type IWhere,
  type Parameter,
  type Scalar,
} from "@deterministic-code/datasources";
import { matches, parametersToObject, parseFilter } from "./filter.ts";

export type MemoryRecord = Record<string, unknown>;

export class MemoryEngine {
  inTransaction = false;
  collection = "";
  records: MemoryRecord[] = [];
  private snapshot?: Map<string, MemoryRecord[]>;
  private readonly inserted = new Map<string, Scalar>();
  readonly collections = new Map<string, MemoryRecord[]>();

  hasCollection(name: string): boolean {
    return this.collections.has(name);
  }

  getCollection(name: string): MemoryRecord[] {
    let rows = this.collections.get(name);
    if (!rows) {
      rows = [];
      this.collections.set(name, rows);
    }
    return rows;
  }

  seed(data: MemoryRecord[] | Record<string, MemoryRecord[]>, collection: string): void {
    if (Array.isArray(data)) {
      if (!this.hasCollection(collection)) {
        this.collections.set(
          collection,
          data.map((row) => ({ ...row })),
        );
      }
      return;
    }
    for (const [name, rows] of Object.entries(data)) {
      if (!this.hasCollection(name)) {
        this.collections.set(
          name,
          rows.map((row) => ({ ...row })),
        );
      }
    }
  }

  begin(): void {
    this.snapshot = new Map(
      [...this.collections].map(([name, rows]) => [name, rows.map((row) => ({ ...row }))]),
    );
    this.inTransaction = true;
  }

  commit(): void {
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  abort(): void {
    if (this.snapshot) {
      this.collections.clear();
      for (const [name, rows] of this.snapshot) {
        this.collections.set(name, rows.map((row) => ({ ...row })));
      }
    }
    this.inTransaction = false;
    this.snapshot = undefined;
  }

  execute(text: string, parameters?: Parameter[]): Response {
    this.records = [];
    const named = namedOp(text);
    if (named) return this.named(named.op, parameters, named.where);
    if (looksJson(text) || !text.trim()) {
      const filter = { ...parseFilter(text), ...parametersToObject(parameters) };
      if (typeof filter.__inserted === "string") {
        const column = typeof filter.__column === "string" ? filter.__column : "id";
        this.records = this.insertedRow(filter.__inserted, column);
        return new Response(null, this.records.length, parameters);
      }
      const collection =
        typeof filter.__table === "string" ? filter.__table : this.collection;
      const { __table: _table, ...rest } = filter;
      this.records = this.getCollection(collection).filter((row) => matches(row, rest));
      return new Response(null, this.records.length, parameters);
    }
    const split = jsonWhere(text);
    let statement;
    try {
      statement = StatementParser.parse(split.sql);
    } catch {
      throw new DatasourceError(`Unsupported Memory command: ${text}`);
    }
    if (statement instanceof InsertStatement) return this.insert(statement, parameters);
    if (statement instanceof UpdateStatement)
      return this.change(statement.table, parameters, split.where ?? statement.where);
    if (statement instanceof DeleteStatement)
      return this.remove(statement.table, parameters, split.where ?? statement.where);
    return this.readSelect(statement as SelectStatement, parameters);
  }

  private named(op: string, parameters: Parameter[] | undefined, whereText?: string): Response {
    const rows = this.getCollection(this.collection);
    const obj = parametersToObject(parameters);
    if (op === "insert") return this.insertRow(obj, rows, this.collection, parameters);
    if (op === "update") {
      const fromParam = typeof obj.$where === "string" ? obj.$where : undefined;
      const next = { ...obj };
      delete next.$where;
      const filter = whereText
        ? parseFilter(whereText)
        : fromParam
          ? parseFilter(fromParam)
          : "id" in next
            ? { id: next.id }
            : next;
      return this.applyUpdate(rows, next, filter, parameters);
    }
    return this.applyDelete(rows, whereText ? parseFilter(whereText) : obj, parameters);
  }

  private insert(insert: InsertStatement, parameters: Parameter[] | undefined): Response {
    const rows = this.rows(insert.table, true);
    return this.insertRow(parametersToObject(parameters), rows, insert.table, parameters);
  }

  private insertRow(
    obj: MemoryRecord,
    rows: MemoryRecord[],
    table: string,
    parameters: Parameter[] | undefined,
  ): Response {
    const row = { ...obj };
    const id = nextId(row, rows);
    row.id = id;
    rows.push(row);
    this.inserted.set(table, id);
    return new Response(id, 1, parameters);
  }

  private change(table: string, parameters: Parameter[] | undefined, where?: IWhere): Response {
    return this.applyChanged(this.rows(table, false), parametersToObject(parameters), where, parameters);
  }

  private remove(table: string, parameters: Parameter[] | undefined, where?: IWhere): Response {
    return this.removeMatched(this.rows(table, false), parametersToObject(parameters), where, parameters);
  }

  private readSelect(select: SelectStatement, parameters: Parameter[] | undefined): Response {
    const rows = this.rows(select.table, false);
    if (select.where instanceof Where) {
      this.records = rows.filter((row) => boundMatch(select.where as Where, row, parameters));
    } else this.records = [...rows];
    return new Response(null, this.records.length, parameters);
  }

  rows(table: string | undefined, create: boolean): MemoryRecord[] {
    return table && table !== "mem" && (create || this.hasCollection(table))
      ? this.getCollection(table)
      : this.getCollection(this.collection);
  }

  private insertedRow(table: string, column: string): MemoryRecord[] {
    const id = this.inserted.get(table);
    return id === undefined ? [] : [{ [column]: id }];
  }

  private applyChanged(
    rows: MemoryRecord[],
    obj: MemoryRecord,
    where: IWhere | undefined,
    parameters: Parameter[] | undefined,
  ): Response {
    if (where instanceof Where) {
      const names = setNames(parameters, where);
      let rowsAffected = 0;
      for (const row of rows) {
        if (!boundMatch(where, row, parameters)) continue;
        for (const name of names) if (name in obj) row[name] = obj[name];
        rowsAffected += 1;
      }
      return new Response(null, rowsAffected, parameters);
    }
    const next = { ...obj };
    delete next.$where;
    const filter = looksJson(renderWhere(where))
      ? parseFilter(renderWhere(where)!)
      : "id" in next
        ? { id: next.id }
        : next;
    return this.applyUpdate(rows, next, filter, parameters);
  }

  private removeMatched(
    rows: MemoryRecord[],
    obj: MemoryRecord,
    where: IWhere | undefined,
    parameters: Parameter[] | undefined,
  ): Response {
    if (where instanceof Where) {
      const remaining = rows.filter((row) => !boundMatch(where, row, parameters));
      const rowsAffected = rows.length - remaining.length;
      rows.length = 0;
      rows.push(...remaining);
      return new Response(null, rowsAffected, parameters);
    }
    const filter = looksJson(renderWhere(where)) ? parseFilter(renderWhere(where)!) : obj;
    return this.applyDelete(rows, filter, parameters);
  }

  private applyUpdate(
    collection: MemoryRecord[],
    obj: MemoryRecord,
    filter: MemoryRecord,
    parameters?: Parameter[],
  ): Response {
    let rowsAffected = 0;
    for (let i = 0; i < collection.length; i += 1) {
      if (matches(collection[i]!, filter)) {
        collection[i] = { ...collection[i], ...obj };
        rowsAffected += 1;
      }
    }
    return new Response(null, rowsAffected, parameters);
  }

  private applyDelete(collection: MemoryRecord[], filter: MemoryRecord, parameters?: Parameter[]): Response {
    const remaining = collection.filter((row) => !matches(row, filter));
    const rowsAffected = collection.length - remaining.length;
    collection.length = 0;
    collection.push(...remaining);
    return new Response(null, rowsAffected, parameters);
  }
}

const nextId = (row: MemoryRecord, rows: MemoryRecord[]): number =>
  typeof row.id === "number"
    ? row.id
    : rows.reduce((max, item) => (typeof item.id === "number" && item.id > max ? item.id : max), 0) + 1;

const boundMatch = (where: Where, row: MemoryRecord, parameters?: Parameter[]): boolean =>
  matchExpr(where.condition, row, parametersToObject(parameters), parameters);

const matchExpr = (
  expression: IExpression,
  row: MemoryRecord,
  bound: MemoryRecord,
  parameters?: Parameter[],
): boolean => {
  if (expression instanceof Operator.And)
    return matchExpr(expression.left, row, bound, parameters) && matchExpr(expression.right, row, bound, parameters);
  if (expression instanceof Operator.Or)
    return matchExpr(expression.left, row, bound, parameters) || matchExpr(expression.right, row, bound, parameters);
  if (expression instanceof Operator.Equals && expression.left instanceof Identifier) {
    const name = expression.left.name;
    const right =
      expression.right instanceof Literal && expression.right.value !== null && expression.right.value !== undefined
        ? expression.right.value
        : bound[name];
    return name in row && row[name] === right;
  }
  if (expression instanceof Operator.In && expression.left instanceof Identifier)
    return inMatch(expression.left.name, expression.right, row, bound);
  if (expression instanceof Operator.NotIn && expression.left instanceof Identifier)
    return !inMatch(expression.left.name, expression.right, row, bound);
  return new ExpressionEvaluator().evaluate(expression, context(row, parameters)) === true;
};

const inMatch = (name: string, right: IExpression, row: MemoryRecord, bound: MemoryRecord): boolean =>
  right instanceof ArrayExpression &&
  name in row &&
  right.elements.some((_, i) => row[name] === bound[`${name}_${i}`]);

const looksJson = (text?: string): boolean => !!text?.trimStart().startsWith("{");

const jsonWhere = (text: string): { sql: string; where?: IWhere } => {
  const mark = text.toLowerCase().lastIndexOf(" where ");
  if (mark < 0) return { sql: text };
  const tail = text.slice(mark + 7).trim();
  if (!looksJson(tail)) return { sql: text };
  return { sql: text.slice(0, mark), where: new WhereText(tail) };
};

const namedOp = (text: string): { op: string; where?: string } | undefined => {
  const trimmed = text.trim();
  const lower = trimmed.toLowerCase();
  if (lower === "insert" || lower === "update" || lower === "delete") return { op: lower };
  for (const op of ["update", "delete"] as const) {
    if (!lower.startsWith(`${op} `)) continue;
    const where = trimmed.slice(op.length).trim();
    if (looksJson(where)) return { op, where };
  }
  return undefined;
};

const renderWhere = (where?: IWhere): string | undefined =>
  where ? DefaultStatementTextRenderer.instance.render(where) : undefined;

const context = (row: MemoryRecord, parameters?: Parameter[]): Record<string, Scalar> => {
  const extras = parametersToObject(parameters);
  return { ...extras, ...row } as Record<string, Scalar>;
};

const setNames = (parameters: Parameter[] | undefined, where: IWhere): string[] => {
  const bound = new Set(where.parameters.map((parameter) => parameter.name));
  const incoming = parameters ?? [];
  return incoming.map((parameter) => parameter.name).filter((name) => !bound.has(name) && !name.endsWith("__where"));
};
