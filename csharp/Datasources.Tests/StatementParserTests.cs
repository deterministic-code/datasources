namespace Datasources.Tests;

public sealed class StatementParserTests
{
    [Theory]
    [InlineData("SELECT id FROM users WHERE name NOT LIKE $name")]
    [InlineData("SELECT id FROM users WHERE age NOT BETWEEN $age_0 AND $age_1")]
    [InlineData("SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5")]
    [InlineData("SELECT \"user\".id AS user_id FROM \"user\"")]
    [InlineData("SELECT id FROM users WHERE 'a' = 'b'")]
    [InlineData("SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id AND profiles.active = $profiles.active")]
    [InlineData("SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)")]
    [InlineData("SELECT id FROM users CROSS JOIN tags")]
    [InlineData("SELECT id FROM users WHERE id IS NULL")]
    [InlineData("SELECT id FROM users WHERE id IS NOT NULL")]
    [InlineData("SELECT id FROM users WHERE name LIKE $name")]
    [InlineData("SELECT id FROM users WHERE id IN ($id_0, $id_1)")]
    [InlineData("SELECT id FROM users WHERE id NOT IN ($id_0)")]
    [InlineData("SELECT id FROM users WHERE age BETWEEN $age_0 AND $age_1")]
    [InlineData("SELECT id FROM users WHERE 1 = 0")]
    [InlineData("SELECT id, name FROM users WHERE id = $id AND age > $age")]
    [InlineData("SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10")]
    public void RoundTrips(string sql) =>
        Assert.Equal(sql, StatementParser.Parse(sql).GetText());

    [Theory]
    [InlineData("INSERT INTO users (id, name) VALUES ($id, $name)")]
    [InlineData("INSERT INTO \"user\" (id) VALUES ($id)")]
    [InlineData("INSERT INTO users (name) VALUES ('ada')")]
    [InlineData("UPDATE users SET name = $name WHERE id = $id")]
    [InlineData("UPDATE users SET name = $name, age = $age")]
    [InlineData("UPDATE users SET name = $name WHERE name NOT LIKE $name")]
    [InlineData("UPDATE users SET name = name WHERE id = $id")]
    [InlineData("DELETE FROM users WHERE id = $id")]
    [InlineData("DELETE FROM users")]
    [InlineData("DELETE FROM \"user\" WHERE id = $id")]
    [InlineData("DELETE FROM users WHERE name NOT LIKE $name")]
    public void RoundTripsWrites(string sql) =>
        Assert.Equal(sql, StatementParser.Parse(sql).GetText());

    [Fact]
    public void BuildsTheWriteTrees()
    {
        Assert.IsType<InsertStatement>(StatementParser.Parse("INSERT INTO users (id, name) VALUES ($id, $name)"));

        var update = Assert.IsType<UpdateStatement>(StatementParser.Parse("UPDATE users SET name = $name WHERE id = $id"));
        Assert.IsType<Operator.Equals>(Assert.IsType<Where>(update.Where).Condition);

        var notLike = Assert.IsType<UpdateStatement>(StatementParser.Parse("UPDATE users SET name = $name WHERE name NOT LIKE $name"));
        Assert.IsType<Operator.NotLike>(Assert.IsType<Where>(notLike.Where).Condition);

        var delete = Assert.IsType<DeleteStatement>(StatementParser.Parse("DELETE FROM users WHERE id = $id"));
        Assert.IsType<Operator.Equals>(Assert.IsType<Where>(delete.Where).Condition);

        var all = Assert.IsType<DeleteStatement>(StatementParser.Parse("DELETE FROM users"));
        Assert.Null(all.Where);
    }

    [Fact]
    public void BuildsTheWhereTree()
    {
        var notLike = Assert.IsType<SelectStatement>(StatementParser.Parse("SELECT id FROM users WHERE name NOT LIKE $name"));
        Assert.IsType<Operator.NotLike>(Assert.IsType<Where>(notLike.Where).Condition);

        var exists = Assert.IsType<SelectStatement>(StatementParser.Parse(
            "SELECT id FROM users WHERE EXISTS (SELECT id FROM posts WHERE posts.user_id = users.id)"));
        Assert.IsType<Operator.Exists>(Assert.IsType<Where>(exists.Where).Condition);

        var full = Assert.IsType<SelectStatement>(StatementParser.Parse(
            "SELECT id FROM users FULL JOIN profiles ON users.id = profiles.user_id"));
        Assert.Equal(JoinKind.Full, Assert.Single(full.Joins).Kind);

        var cross = Assert.IsType<SelectStatement>(StatementParser.Parse("SELECT id FROM users CROSS JOIN tags"));
        var join = Assert.Single(cross.Joins);
        Assert.Equal(JoinKind.Cross, join.Kind);
        Assert.Null(join.On);

        var shaped = Assert.IsType<SelectStatement>(StatementParser.Parse(
            "SELECT DISTINCT status FROM orders GROUP BY status HAVING status = $status ORDER BY status ASC LIMIT 10 OFFSET 5"));
        Assert.True(shaped.Distinct);
        Assert.Equal(10, shaped.Limit);
        Assert.Equal(5, shaped.Offset);
        Assert.IsType<Operator.Equals>(Assert.IsType<Where>(shaped.Having).Condition);
    }

    [Fact]
    public void KeepsAStringBesideAColumnAsItsParameter()
    {
        var query = Assert.IsType<SelectStatement>(StatementParser.Parse("SELECT id FROM users WHERE name = 'ada'"));
        Assert.Equal("SELECT id FROM users WHERE name = $name", query.GetText());
        Assert.Equal("ada", Assert.IsType<Value.Text>(Assert.Single(query.Where!.Parameters).Value).Item);
    }

    [Fact]
    public void EvaluatesTheNewPredicates()
    {
        var row = new Row { ["name"] = "ada", ["age"] = 30, ["id"] = Value.Null.Instance };
        var evaluator = new ExpressionEvaluator();
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new Operator.IsNull(new Identifier("id")), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new Operator.Like("name", "a%"), row));
        Assert.Equal(new Value.Bool(false), evaluator.Evaluate(new Operator.NotLike("name", "a%"), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new Operator.Between("age", 18, 40), row));
        Assert.Equal(new Value.Bool(false), evaluator.Evaluate(new Operator.NotBetween("age", 18, 40), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new Operator.NotIn("age", [1, 2]), row));
        Assert.Equal("name NOT LIKE $name", new Where(new Operator.NotLike("name", "%a")).GetText());
        Assert.Equal("age NOT BETWEEN $age_0 AND $age_1", new Where(new Operator.NotBetween("age", 1, 2)).GetText());
        Assert.Equal("1 = 1", new Where(new Operator.NotIn("id", [])).GetText());
        Assert.Equal("NOT (name LIKE $name)", new Where(new Operator.Not(new Operator.Like("name", "%a"))).GetText());
    }
}
