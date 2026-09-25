import { describe, expect, it } from "vitest";
import {
  ArrayExpression,
  DeleteStatement,
  Identifier,
  InsertStatement,
  Join,
  Literal,
  Operator,
  QueryStatement,
  StatementEvaluator,
  StatementParser,
  Subquery,
  UpdateStatement,
  WhereClause,
} from "../index.ts";
import type { IExpression } from "./IExpression.ts";

const roundTrip = (sql: string): void => {
  expect(StatementParser.parse(sql).getText()).toBe(sql);
};

describe("StatementParser", () => {
  it("round-trips the select grammar", () => {
    for (const sql of [
      "SELECT id FROM users WHERE name NOT LIKE $name",
      "SELECT id FROM users WHERE age NOT BETWEEN $age_0 AND $age_1",
      "SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5",
      'SELECT "user".id AS user_id FROM "user"',
      "SELECT id FROM users WHERE 'a' = 'b'",
      "SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id AND profiles.active = $profiles.active",
      "SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)",
      "SELECT id FROM users CROSS JOIN tags",
      "SELECT id FROM users WHERE id IS NULL",
      "SELECT id FROM users WHERE id IS NOT NULL",
      "SELECT id FROM users WHERE name LIKE $name",
      "SELECT id FROM users WHERE id IN ($id_0, $id_1)",
      "SELECT id FROM users WHERE id NOT IN ($id_0)",
      "SELECT id FROM users WHERE age BETWEEN $age_0 AND $age_1",
      "SELECT id FROM users WHERE 1 = 0",
      "SELECT id, name FROM users WHERE id = $id AND age > $age",
      "SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10",
    ])
      roundTrip(sql);
  });

  it("round-trips the write grammar", () => {
    for (const sql of [
      "INSERT INTO users (id, name) VALUES ($id, $name)",
      'INSERT INTO "user" (id) VALUES ($id)',
      "INSERT INTO users (name) VALUES ('ada')",
      "UPDATE users SET name = $name WHERE id = $id",
      "UPDATE users SET name = $name, age = $age",
      "UPDATE users SET name = $name WHERE name NOT LIKE $name",
      "UPDATE users SET name = name WHERE id = $id",
      "DELETE FROM users WHERE id = $id",
      "DELETE FROM users",
      'DELETE FROM "user" WHERE id = $id',
      "DELETE FROM users WHERE name NOT LIKE $name",
    ])
      roundTrip(sql);
  });

  it("builds the write trees", () => {
    expect(StatementParser.parse("INSERT INTO users (id, name) VALUES ($id, $name)")).toBeInstanceOf(
      InsertStatement,
    );
    const update = StatementParser.parse("UPDATE users SET name = $name WHERE id = $id") as UpdateStatement;
    expect((update.where as WhereClause).condition).toBeInstanceOf(Operator.Equals);
    const notLike = StatementParser.parse(
      "UPDATE users SET name = $name WHERE name NOT LIKE $name",
    ) as UpdateStatement;
    expect((notLike.where as WhereClause).condition).toBeInstanceOf(Operator.NotLike);
    const deleted = StatementParser.parse("DELETE FROM users WHERE id = $id") as DeleteStatement;
    expect((deleted.where as WhereClause).condition).toBeInstanceOf(Operator.Equals);
    expect((StatementParser.parse("DELETE FROM users") as DeleteStatement).where).toBeUndefined();
  });

  it("builds the where tree", () => {
    const notLike = StatementParser.parse("SELECT id FROM users WHERE name NOT LIKE $name") as QueryStatement;
    expect((notLike.where as WhereClause).condition).toBeInstanceOf(Operator.NotLike);

    const exists = StatementParser.parse(
      "SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)",
    ) as QueryStatement;
    expect((exists.where as WhereClause).condition).toBeInstanceOf(Operator.Exists);

    const full = StatementParser.parse(
      "SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id",
    ) as QueryStatement;
    expect(full.joins[0]?.kind).toBe("full");

    const cross = StatementParser.parse("SELECT id FROM users CROSS JOIN tags") as QueryStatement;
    expect(cross.joins[0]?.kind).toBe("cross");
    expect(cross.joins[0]?.on).toBeUndefined();

    const shaped = StatementParser.parse(
      "SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5",
    ) as QueryStatement;
    expect(shaped.distinct).toBe(true);
    expect(shaped.limit).toBe(10);
    expect(shaped.offset).toBe(5);
    expect((shaped.having as WhereClause).condition).toBeInstanceOf(Operator.Equals);
  });

  it("keeps a string beside a column as its parameter", () => {
    const query = StatementParser.parse("SELECT id FROM users WHERE name = 'ada'") as QueryStatement;
    expect(query.getText()).toBe("SELECT id FROM users WHERE name = $name");
    expect(query.where?.parameters).toEqual([{ name: "name", value: "ada" }]);
  });

  it("evaluates the new predicates", () => {
    const row = { name: "ada", age: 30, id: null };
    const evaluator = new StatementEvaluator();
    expect(evaluator.evaluate(new Operator.IsNull(new Identifier("id")), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Like("name", "a%"), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.NotLike("name", "a%"), row)).toBe(false);
    expect(evaluator.evaluate(new Operator.Between("age", 18, 40), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.NotBetween("age", 18, 40), row)).toBe(false);
    expect(evaluator.evaluate(new Operator.NotIn("age", [1, 2]), row)).toBe(true);
    expect(new WhereClause(new Operator.NotLike("name", "%a")).getText()).toBe("name NOT LIKE $name");
    expect(new WhereClause(new Operator.NotBetween("age", 1, 2)).getText()).toBe(
      "age NOT BETWEEN $age_0 AND $age_1",
    );
    expect(new WhereClause(new Operator.NotIn("id", [])).getText()).toBe("1 = 1");
    expect(new WhereClause(new Operator.Not(new Operator.Like("name", "%a"))).getText()).toBe(
      "NOT (name LIKE $name)",
    );
  });

  it("parses the remaining select forms", () => {
    for (const sql of [
      "SELECT id FROM (SELECT id FROM users) AS u",
      "SELECT id FROM users LEFT JOIN posts ON posts.user_id = users.id",
      "SELECT id FROM users RIGHT JOIN tags ON tags.id = users.id",
      "SELECT id FROM users WHERE NOT (name LIKE $name)",
      "SELECT id FROM users WHERE id <> $id",
      "SELECT id FROM users WHERE age <= $age",
      "SELECT id FROM users WHERE age >= $age",
      "SELECT id FROM users WHERE age < $age",
      "SELECT id FROM users WHERE age > $age",
      "SELECT id FROM users WHERE true",
      "SELECT id FROM users WHERE false",
      "SELECT id FROM users WHERE null",
      "SELECT id FROM users WHERE 'o''brien' = 'x'",
      'SELECT id FROM "a""b"',
      "SELECT id FROM users WHERE id IN (SELECT id FROM posts)",
      "SELECT id FROM users WHERE id NOT IN (SELECT id FROM posts)",
      "SELECT id FROM users WHERE (SELECT id FROM posts) = id",
      "SELECT MIN(n), MAX(amount) FROM orders",
      "SELECT COUNT FROM users",
      "SELECT id FROM users WHERE id = $id OR age = $age",
      "SELECT id FROM users OFFSET 3",
      "SELECT id FROM users GROUP BY a, b",
      "SELECT * FROM users",
    ])
      roundTrip(sql);
    expect(StatementParser.parse("SELECT id FROM users WHERE (id = $id)").getText()).toBe(
      "SELECT id FROM users WHERE id = $id",
    );
    expect(() => StatementParser.parse("INSERT INTO users")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users extra")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users WHERE 'ab")).toThrow("empty statement");
    expect(() => StatementParser.parse('SELECT id FROM "user')).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users WHERE id NOT = 1")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users LIMIT")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users WHERE id IS")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM users WHERE (id = $id")).toThrow("empty statement");
    expect(() => StatementParser.parse("SELECT id FROM 1")).toThrow("empty statement");
  });

  it("rewrites and collects the new operators", () => {
    const same = (name: string): string => name;
    const keep = <T>(_name: string, cell: T): T => cell;
    const nested = new QueryStatement({
      table: "posts",
      fields: ["id"],
      where: new WhereClause(new Operator.Equals("id", 1)),
      having: new WhereClause(new Operator.Equals("n", 2)),
      joins: [Join.inner("tags", "posts.id", "tags.post_id")],
    });
    const plain = new QueryStatement("SELECT 1");
    const mapped = new WhereClause(
      new Operator.And(
        new Operator.Or(new Operator.IsNull(new Identifier("id")), new Operator.IsNotNull(new Identifier("id"))),
        new Operator.Not(new Operator.Like(new Literal("a"), new Identifier("name"))),
      ),
    ).map(same, keep);
    expect(mapped.getText()).toBe("(id IS NULL OR id IS NOT NULL) AND (NOT ('a' LIKE name))");
    for (const op of [
      new Operator.NotEquals("id", 1),
      new Operator.GreaterThan("id", 1),
      new Operator.LessThan("id", 1),
      new Operator.GreaterThanOrEqual("id", 1),
      new Operator.LessThanOrEqual("id", 1),
      new Operator.Like("name", "a"),
      new Operator.NotLike("name", "a"),
      new Operator.Between("age", 1, 2),
      new Operator.NotBetween("age", 1, 2),
      new Operator.In("id", [1]),
      new Operator.NotIn("id", [1]),
      new Operator.Exists(new Subquery(nested)),
      new Operator.Exists(new Subquery(plain)),
    ])
      expect(new WhereClause(op).map(same, keep).getText().length).toBeGreaterThan(0);
    expect(new WhereClause(new Operator.Equals('"user"', 1)).getText()).toBe('"user" = $user');
    expect(new WhereClause(new Operator.Equals('"a.b"', 1)).parameters).toEqual([
      { name: "a.b", value: 1 },
    ]);
    expect(new WhereClause(new Operator.In(new Identifier("id"), new Subquery(nested))).getText()).toBe(
      "id IN (SELECT id FROM posts INNER JOIN tags ON posts.id = tags.post_id WHERE id = $id HAVING n = $n)",
    );
    expect(
      new WhereClause(new Operator.NotIn(new Identifier("id"), new Subquery(nested))).parameters,
    ).toEqual([
      { name: "id", value: 1 },
      { name: "n", value: 2 },
    ]);
    expect(
      new WhereClause(new Operator.Between(new Literal(1), new Literal(0), new Literal(2))).getText(),
    ).toBe("1 BETWEEN 0 AND 2");
    expect(new WhereClause(new Operator.NotBetween(new Literal(1), new Literal(0), new Literal(2))).getText()).toBe(
      "1 NOT BETWEEN 0 AND 2",
    );
    expect(new WhereClause(new Operator.NotBetween(new Literal(1), new Literal(0), new Literal(2))).parameters).toEqual(
      [],
    );
    expect(
      new WhereClause(
        new Operator.Between(new Identifier("age"), new Identifier("low"), new Identifier("high")),
      ).map(same, keep).getText(),
    ).toBe("age BETWEEN low AND high");
    expect(
      new WhereClause(new Operator.In(new Identifier("id"), new Subquery(plain))).map(same, keep).getText(),
    ).toBe("id IN (SELECT 1)");
    const bare = new QueryStatement({ table: "posts", fields: ["id"], joins: [Join.cross("tags")] });
    expect(new WhereClause(new Operator.Exists(new Subquery(bare))).map(same, keep).getText()).toBe(
      "EXISTS (SELECT id FROM posts CROSS JOIN tags)",
    );
    expect(new WhereClause(new Operator.Exists(new Subquery(bare))).parameters).toEqual([]);
    expect(new WhereClause(new Operator.Exists(new Subquery(nested))).parameters).toEqual([
      { name: "id", value: 1 },
      { name: "n", value: 2 },
    ]);
    expect(new WhereClause(new Operator.NotIn("id", [1, 2])).parameters).toEqual([
      { name: "id_0", value: 1 },
      { name: "id_1", value: 2 },
    ]);
    expect(new WhereClause(new Operator.Between("age", 1, 2)).parameters).toEqual([
      { name: "age_0", value: 1 },
      { name: "age_1", value: 2 },
    ]);
    expect(new WhereClause(new Operator.NotBetween("age", 1, 2)).parameters).toEqual([
      { name: "age_0", value: 1 },
      { name: "age_1", value: 2 },
    ]);
    expect(
      new WhereClause(
        new Operator.In(new Identifier("id"), new ArrayExpression([new Identifier("x")])),
      ).map(same, keep).getText(),
    ).toBe("id IN ($id_0)");
    expect(new WhereClause(new WhereClause(new Operator.Equals("id", 1))).map(same, keep).getText()).toBe(
      "id = $id",
    );
    expect(new WhereClause(new ArrayExpression([new Literal(1)])).map(same, keep).getText()).toBe("1");
    expect(Join.full("profiles", "users.id", "profiles.user_id").kind).toBe("full");
    expect(() =>
      new QueryStatement({ table: "users", fields: ["id"], joins: [{ kind: "inner", table: "t" }] }).getText(),
    ).toThrow("empty join");
    expect(() => new QueryStatement({ table: "users", fields: ["id"], offset: -1 })).toThrow(
      "negative offset",
    );
    expect(() => new WhereClause(new Ghost() as IExpression).map(same, keep)).toThrow("empty expression");
  });

  it("evaluates null patterns and rejects a subquery", () => {
    const row = { name: "ada", age: 30, id: null };
    const evaluator = new StatementEvaluator();
    expect(evaluator.evaluate(new Operator.Like("name", null), row)).toBe(false);
    expect(evaluator.evaluate(new Operator.NotLike("name", null), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.IsNotNull(new Identifier("name")), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Like("name", "a_a"), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Like("name", "a%"), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Like("name", "z%"), row)).toBe(false);
    expect(evaluator.evaluate(new Operator.Like("name", "___"), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Like("name", "ada%"), row)).toBe(true);
    expect(evaluator.evaluate(new Operator.Between("age", null, 40), row)).toBe(false);
    expect(() => evaluator.evaluate(new Operator.Like("age", 1), row)).toThrow("empty like");
    expect(() => evaluator.evaluate(new Operator.NotIn(new Identifier("id"), new Literal(1)), row)).toThrow(
      "empty in",
    );
    const query = new Subquery(new QueryStatement("SELECT 1"));
    expect(() => evaluator.evaluate(new Operator.Exists(query), row)).toThrow("subquery is not evaluable");
    expect(() => evaluator.evaluate(query, row)).toThrow("subquery is not evaluable");
  });
});

class Ghost {}
