import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import type { IStatement } from "./IStatement.ts";
import { Join, type JoinKind } from "./Join.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import { OrderBy } from "./OrderBy.ts";
import { QueryStatement } from "./QueryStatement.ts";
import { Subquery } from "./Subquery.ts";
import { WhereClause } from "./WhereClause.ts";

const empty = (): never => {
  throw new Error("empty statement");
};

class Cursor {
  private i = 0;

  constructor(private readonly text: string) {}

  statement(): IStatement {
    return this.peekWord("SELECT") ? this.selectStatement() : empty();
  }

  end(): void {
    this.skip();
    if (this.i !== this.text.length) empty();
  }

  private selectStatement(): QueryStatement {
    this.expectWord("SELECT");
    const distinct = this.tryWord("DISTINCT");
    const fields = this.selectList();
    this.expectWord("FROM");
    const table = this.fromItem();
    const joins = this.joins();
    const where = this.tryWord("WHERE") ? new WhereClause(this.expr()) : undefined;
    const groupBy = this.tryWord("GROUP") ? this.group() : undefined;
    const having = this.tryWord("HAVING") ? new WhereClause(this.expr()) : undefined;
    const orderBy = this.tryWord("ORDER") ? this.order() : undefined;
    const limit = this.tryWord("LIMIT") ? this.integer() : undefined;
    const offset = this.tryWord("OFFSET") ? this.integer() : undefined;
    return new QueryStatement({
      distinct,
      table,
      fields,
      where,
      joins,
      groupBy,
      having,
      orderBy,
      limit,
      offset,
    });
  }

  private selectList(): string[] {
    if (this.tryChar("*")) return [];
    const fields = [this.selectItem()];
    while (this.tryChar(",")) fields.push(this.selectItem());
    return fields;
  }

  private selectItem(): string {
    for (const name of ["COUNT", "SUM", "MIN", "MAX"]) {
      const mark = this.i;
      if (!this.tryWord(name) || !this.tryChar("(")) {
        this.i = mark;
        continue;
      }
      const arg = this.tryChar("*") ? "*" : this.ident();
      this.expectChar(")");
      const alias = this.tryWord("AS") ? ` AS ${this.ident()}` : "";
      return `${name}(${arg})${alias}`;
    }
    const column = this.ident();
    return this.tryWord("AS") ? `${column} AS ${this.ident()}` : column;
  }

  private fromItem(): string {
    if (!this.peek("(")) return this.ident();
    const nested = this.nested();
    this.expectWord("AS");
    return `(${nested.getText()}) AS ${this.ident()}`;
  }

  private nested(): QueryStatement {
    this.expectChar("(");
    const query = this.selectStatement();
    this.expectChar(")");
    return query;
  }

  private joins(): Join[] {
    const joins: Join[] = [];
    for (let kind = this.kind(); kind; kind = this.kind()) {
      this.expectWord(
        kind === "cross"
          ? "CROSS"
          : kind === "inner"
            ? "INNER"
            : kind === "left"
              ? "LEFT"
              : kind === "right"
                ? "RIGHT"
                : "FULL",
      );
      this.expectWord("JOIN");
      const table = this.fromItem();
      joins.push(kind === "cross" ? Join.cross(table) : { kind, table, on: this.on() });
    }
    return joins;
  }

  private kind(): JoinKind | undefined {
    return this.peekWord("CROSS")
      ? "cross"
      : this.peekWord("INNER")
        ? "inner"
        : this.peekWord("LEFT")
          ? "left"
          : this.peekWord("RIGHT")
            ? "right"
            : this.peekWord("FULL")
              ? "full"
              : undefined;
  }

  private on(): IExpression {
    this.expectWord("ON");
    return this.expr();
  }

  private group(): string[] {
    this.expectWord("BY");
    return this.list(() => this.ident());
  }

  private order(): OrderBy[] {
    this.expectWord("BY");
    return this.list(() => this.orderItem());
  }

  private orderItem(): OrderBy {
    const column = this.ident();
    if (this.tryWord("DESC")) return OrderBy.desc(column);
    this.expectWord("ASC");
    return OrderBy.asc(column);
  }

  private list<T>(item: () => T): T[] {
    const items = [item()];
    while (this.tryChar(",")) items.push(item());
    return items;
  }

  private expr(): IExpression {
    let left = this.and();
    while (this.tryWord("OR")) left = new Operator.Or(left, this.and());
    return left;
  }

  private and(): IExpression {
    let left = this.not();
    while (this.tryWord("AND")) left = new Operator.And(left, this.not());
    return left;
  }

  private not(): IExpression {
    if (!this.peekWord("NOT") || !this.peekAfter("NOT", "(")) return this.predicate();
    this.expectWord("NOT");
    this.expectChar("(");
    const inner = this.expr();
    this.expectChar(")");
    return new Operator.Not(inner);
  }

  private predicate(): IExpression {
    if (this.tryWord("EXISTS")) return new Operator.Exists(new Subquery(this.nested()));
    const left = this.primary();
    if (this.tryWord("IS")) return this.nullTest(left);
    if (this.tryWord("NOT")) return this.negated(left);
    if (this.tryWord("IN")) return new Operator.In(left, this.inList());
    if (this.tryWord("BETWEEN")) return this.bounds(left, false);
    if (this.tryWord("LIKE")) return new Operator.Like(left, this.param());
    if (this.tryText("<>")) return new Operator.NotEquals(left, this.primary());
    if (this.tryText("<=")) return new Operator.LessThanOrEqual(left, this.primary());
    if (this.tryText(">=")) return new Operator.GreaterThanOrEqual(left, this.primary());
    if (this.tryChar("=")) return new Operator.Equals(left, this.primary());
    if (this.tryChar("<")) return new Operator.LessThan(left, this.primary());
    if (this.tryChar(">")) return new Operator.GreaterThan(left, this.primary());
    return left;
  }

  private nullTest(left: IExpression): IExpression {
    if (this.tryWord("NOT")) {
      this.expectWord("NULL");
      return new Operator.IsNotNull(left);
    }
    this.expectWord("NULL");
    return new Operator.IsNull(left);
  }

  private negated(left: IExpression): IExpression {
    if (this.tryWord("IN")) return new Operator.NotIn(left, this.inList());
    if (this.tryWord("BETWEEN")) return this.bounds(left, true);
    this.expectWord("LIKE");
    return new Operator.NotLike(left, this.param());
  }

  private bounds(left: IExpression, not: boolean): IExpression {
    const low = this.param();
    this.expectWord("AND");
    const high = this.param();
    return not ? new Operator.NotBetween(left, low, high) : new Operator.Between(left, low, high);
  }

  private inList(): IExpression {
    this.expectChar("(");
    if (this.peekWord("SELECT")) {
      const query = this.selectStatement();
      this.expectChar(")");
      return new Subquery(query);
    }
    const items = [this.param()];
    while (this.tryChar(",")) items.push(this.param());
    this.expectChar(")");
    return new ArrayExpression(items);
  }

  private primary(): IExpression {
    if (this.tryChar("(")) {
      if (this.peekWord("SELECT")) {
        const query = this.selectStatement();
        this.expectChar(")");
        return new Subquery(query);
      }
      const inner = this.expr();
      this.expectChar(")");
      return inner;
    }
    if (this.tryChar("$")) {
      this.ident();
      return new Literal(null);
    }
    if (this.peekDigit()) return new Literal(this.integer());
    if (this.tryWord("true")) return new Literal(true);
    if (this.tryWord("false")) return new Literal(false);
    if (this.tryWord("null")) return new Literal(null);
    if (this.peek("'")) return this.string();
    return new Identifier(this.ident());
  }

  private param(): IExpression {
    this.expectChar("$");
    this.ident();
    return new Literal(null);
  }

  private string(): IExpression {
    this.expectChar("'");
    let value = "";
    while (this.i < this.text.length) {
      if (this.text[this.i] === "'") {
        if (this.text[this.i + 1] === "'") {
          value += "'";
          this.i += 2;
          continue;
        }
        this.i++;
        return new Literal(value);
      }
      value += this.text[this.i++];
    }
    return empty();
  }

  private ident(): string {
    let name = this.part();
    while (this.tryChar(".")) name += `.${this.part()}`;
    return name;
  }

  private part(): string {
    this.skip();
    return this.peek('"') ? this.quoted() : this.name();
  }

  private quoted(): string {
    this.expectChar('"');
    let name = '"';
    while (this.i < this.text.length) {
      if (this.text[this.i] === '"') {
        if (this.text[this.i + 1] === '"') {
          name += '""';
          this.i += 2;
          continue;
        }
        this.i++;
        return `${name}"`;
      }
      name += this.text[this.i++];
    }
    return empty();
  }

  private name(): string {
    this.skip();
    if (this.i >= this.text.length || !/[A-Za-z]/.test(this.at(this.i))) empty();
    const start = this.i++;
    while (this.i < this.text.length && /[A-Za-z0-9_]/.test(this.at(this.i))) this.i++;
    return this.text.slice(start, this.i);
  }

  private integer(): number {
    this.skip();
    const start = this.i;
    while (this.i < this.text.length && /[0-9]/.test(this.at(this.i))) this.i++;
    if (start === this.i) empty();
    return Number(this.text.slice(start, this.i));
  }

  private peekDigit(): boolean {
    this.skip();
    return /[0-9]/.test(this.at(this.i));
  }

  private peek(token: string): boolean {
    this.skip();
    return this.text[this.i] === token;
  }

  private peekWord(word: string): boolean {
    const mark = this.i;
    const matched = this.tryWord(word);
    this.i = mark;
    return matched;
  }

  private peekAfter(word: string, token: string): boolean {
    const mark = this.i;
    const matched = this.tryWord(word) && this.peek(token);
    this.i = mark;
    return matched;
  }

  private tryWord(word: string): boolean {
    this.skip();
    if (this.i + word.length > this.text.length) return false;
    if (this.text.slice(this.i, this.i + word.length).toLowerCase() !== word.toLowerCase()) return false;
    const after = this.i + word.length;
    if (/[A-Za-z0-9_]/.test(this.at(after))) return false;
    this.i = after;
    return true;
  }

  private tryChar(token: string): boolean {
    this.skip();
    if (this.text[this.i] !== token) return false;
    this.i++;
    return true;
  }

  private tryText(token: string): boolean {
    this.skip();
    if (!this.text.startsWith(token, this.i)) return false;
    this.i += token.length;
    return true;
  }

  private expectChar(token: string): void {
    if (!this.tryChar(token)) empty();
  }

  private expectWord(word: string): void {
    if (!this.tryWord(word)) empty();
  }

  private skip(): void {
    while (this.i < this.text.length && /\s/.test(this.at(this.i))) this.i++;
  }

  private at(index: number): string {
    return this.text[index] ?? "";
  }
}

export class StatementParser {
  static parse(text: string): IStatement {
    const cursor = new Cursor(text);
    const statement = cursor.statement();
    cursor.end();
    return statement;
  }
}
