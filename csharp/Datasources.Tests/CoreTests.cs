using Datasources;

namespace Datasources.Tests;

public class CoreTests
{
    static readonly string[] EnvKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA", "DEFAULT_PROVIDER", "DEFAULT_DATASOURCE_URL", "PROVIDER"];

    public CoreTests()
    {
        DatasourceFactory.Reset();
        foreach (var key in EnvKeys)
            Environment.SetEnvironmentVariable(key, null);
    }

    [Fact]
    public void Errors()
    {
        var missing = new ProviderNotInstalledException("mysql");
        Assert.Equal("mysql", missing.Provider);
        Assert.Contains("mysql", missing.Message);
        var unimplemented = new ProviderNotImplementedException("oracle");
        Assert.Equal("oracle", unimplemented.Provider);
        Assert.Contains("oracle", unimplemented.Message);
        Assert.Equal("broken", new DatasourceException("broken").Message);
    }

    [Fact]
    public void HelpersAndQuery()
    {
        Assert.Empty(Helpers.ParametersToRecord(null));
        Assert.Equal(new Value.Int(1), Helpers.ParametersToRecord(new[] { new Parameter("id", 1) })["id"]);
        var select = new QueryStatement("SELECT 1");
        var byId = new Where("id = 1");
        var withWhere = new QueryStatement("SELECT 1", byId);
        Assert.Equal("SELECT 1", select.GetText());
        Assert.Null(select.Where);
        Assert.Same(byId, withWhere.Where);
        Assert.Equal("id = 1", byId.GetText());
        var withId = new Row { ["id"] = 9, ["name"] = "a" };
        Assert.Single(Helpers.IdentityFilter(withId));
        Assert.Equal("id = $id", Helpers.IdentityWhereText(withId));
        var noId = new Row { ["name"] = "a", ["city"] = "x" };
        Assert.Equal(2, Helpers.IdentityFilter(noId).Count);
        Assert.Contains("name = $name", Helpers.IdentityWhereText(noId));
        Assert.Equal("1 = 1", Helpers.IdentityWhereText(new Row()));
    }

    [Fact]
    public void StatementsAndBuilders()
    {
        var where = new WhereClause(new Operator.And(
            new Operator.Equals("id", 1),
            new Operator.GreaterThan("age", 18)));
        Assert.Equal("id = $id AND age > $age", where.GetText());
        Assert.Equal(2, where.Parameters.Count);
        Assert.Equal(new Value.Int(1), where.Parameters[0].Value);
        Assert.Equal(new Value.Int(18), where.Parameters[1].Value);
        Assert.Equal("id <> $id", new WhereClause(new Operator.NotEquals("id", 2)).GetText());
        Assert.Equal("n < $n", new WhereClause(new Operator.LessThan("n", 0)).GetText());
        Assert.Equal("n >= $n", new WhereClause(new Operator.GreaterThanOrEqual("n", 0)).GetText());
        Assert.Equal("n <= $n", new WhereClause(new Operator.LessThanOrEqual("n", 0)).GetText());
        Assert.Equal("1 = 1", new WhereClause(new Operator.Equals(new Literal(1), new Literal(1))).GetText());
        Assert.Equal("1 = 0", new WhereClause(new Operator.Equals(new Literal(1), new Literal(0))).GetText());
        Assert.Equal("id = $id", new WhereClause(new Operator.Equals("id", 1)).GetText());
        Assert.Equal(
            "(id = $id AND age > $age) OR name = $name",
            new WhereClause(new Operator.Or(where, new Operator.Equals("name", "a"))).GetText());
        Assert.Equal(3, new WhereClause(new Operator.Or(where, new Operator.Equals("name", "a"))).Parameters.Count);
        var notWhere = new WhereClause(new Operator.Not(new WhereClause(new Operator.Equals("id", 1))));
        Assert.Equal("NOT (id = $id)", notWhere.GetText());
        Assert.Equal(["id"], notWhere.Parameters.Select(parameter => parameter.Name));
        Assert.Equal(
            "(id = $id OR id = $id) AND age = $age",
            new WhereClause(new Operator.And(
                new Operator.Or(new Operator.Equals("id", 1), new Operator.Equals("id", 2)),
                new Operator.Equals("age", 18))).GetText());
        Assert.Equal(
            "(id IN ($id_0)) OR name = $name",
            new WhereClause(new Operator.Or(
                new Operator.In("id", [1]),
                new Operator.Equals("name", "a"))).GetText());
        Assert.Equal("id", new WhereClause(new Identifier("id")).GetText());
        Assert.Empty(new WhereClause(new Identifier("id")).Parameters);
        Assert.Equal("1", new WhereClause(new ArrayExpression([new Literal(1)])).GetText());
        Assert.Empty(new WhereClause(new ArrayExpression([new Literal(1)])).Parameters);
        Assert.Equal("a = b", new WhereClause(new Operator.Equals(new Identifier("a"), new Identifier("b"))).GetText());
        Assert.Empty(new WhereClause(new Operator.Equals(new Identifier("a"), new Identifier("b"))).Parameters);
        Assert.Equal("1 IN 2", new WhereClause(new Operator.In(new Literal(1), new Literal(2))).GetText());
        Assert.Equal("true = false", new WhereClause(new Operator.Equals(new Literal(true), new Literal(false))).GetText());
        Assert.Empty(new WhereClause(new Operator.Equals(new Literal(true), new Literal(false))).Parameters);
        Assert.Equal("null = null", new WhereClause(new Operator.Equals(new Literal(Value.Null.Instance), new Literal(Value.Null.Instance))).GetText());
        Assert.Equal("1.5 = 2.5", new WhereClause(new Operator.Equals(new Literal(1.5), new Literal(2.5))).GetText());
        Assert.Equal("1.5 = 2.5", new WhereClause(new Operator.Equals(new Literal(1.5m), new Literal(2.5m))).GetText());
        Assert.Equal("'a' = 'b'", new WhereClause(new Operator.Equals(new Literal("a"), new Literal("b"))).GetText());
        Assert.Contains("=", new WhereClause(new Operator.Equals(new Literal(Guid.Empty), new Literal(Guid.Empty))).GetText());
        Assert.Contains("=", new WhereClause(new Operator.Equals(new Literal(new DateTime(2024, 1, 2, 0, 0, 0, DateTimeKind.Utc)), new Literal(new DateTime(2024, 1, 2, 0, 0, 0, DateTimeKind.Utc)))).GetText());
        Assert.Contains("=", new WhereClause(new Operator.Equals(new Literal(new Value.Binary([1])), new Literal(new Value.Binary([1])))).GetText());
        Assert.Equal(
            "(NOT (id = $id)) AND age = $age",
            new WhereClause(new Operator.And(
                new Operator.Not(new Operator.Equals("id", 1)),
                new Operator.Equals("age", 18))).GetText());
        Assert.Equal("id = $id", new WhereClause(new Operator.Equals("id", null)).GetText());
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.Equals("id", null)).Parameters[0].Value);
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.NotEquals("id", null)).Parameters[0].Value);
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.GreaterThan("id", null)).Parameters[0].Value);
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.LessThan("id", null)).Parameters[0].Value);
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.GreaterThanOrEqual("id", null)).Parameters[0].Value);
        Assert.Equal(Value.Null.Instance, new WhereClause(new Operator.LessThanOrEqual("id", null)).Parameters[0].Value);
        Assert.Throws<InvalidOperationException>(() => new WhereClause(new Where("x")).GetText());
        Assert.Throws<InvalidOperationException>(() => new WhereClause(new Where("x")).Parameters);
        Assert.Throws<InvalidOperationException>(() =>
            new WhereClause(new Operator.In(new Identifier("id"), new ArrayExpression([new Identifier("x")]))).Parameters);
        Assert.Equal("1 = 0", new WhereClause(new Operator.In("id", [])).GetText());
        Assert.Empty(new WhereClause(new Operator.In("id", [])).Parameters);
        var inIds = new WhereClause(new Operator.In("id", [1, 2]));
        Assert.Equal("id IN ($id_0, $id_1)", inIds.GetText());
        Assert.Equal(["id_0", "id_1"], inIds.Parameters.Select(parameter => parameter.Name));
        Assert.Equal(new Value.Int(1), inIds.Parameters[0].Value);
        Assert.Equal(new Value.Int(2), inIds.Parameters[1].Value);
        var and = Assert.IsType<Operator.And>(where.Condition);
        var idEquals = Assert.IsType<Operator.Equals>(and.Left);
        Assert.Equal("id", Assert.IsType<Identifier>(idEquals.Left).Name);
        Assert.Equal(new Value.Int(1), Assert.IsType<Literal>(idEquals.Right).Value);

        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var query = new QueryStatement(new() { Table = "users", Fields = ["id", "name"], Where = where });
        Assert.Equal("SELECT id, name FROM users WHERE id = $id AND age > $age", query.GetText());
        Assert.Same(where, query.Where);
        Assert.Equal("SELECT * FROM users", new QueryStatement(new() { Table = "users", Fields = [] }).GetText());
        Assert.Null(new QueryStatement(new() { Table = "users", Fields = [] }).Where);
        Assert.Equal("SELECT id, name FROM users", new QueryStatement(new() { Table = "users", Fields = ["id", "name"] }).GetText());
        Assert.Equal(
            "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id",
            new QueryStatement(new()
            {
                Table = "users",
                Fields = ["users.id", "orders.total"],
                Joins = [Join.Inner("orders", "users.id", "orders.user_id")],
            }).GetText());
        Assert.Equal(
            "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
            new QueryStatement(new()
            {
                Table = "users",
                Fields = ["users.id"],
                Where = new WhereClause(new Operator.Equals("users.id", 1)),
                Joins =
                [
                    Join.Inner("orders", "users.id", "orders.user_id"),
                    Join.Left("payments", "orders.id", "payments.order_id"),
                    Join.Right("profiles", "users.id", "profiles.user_id"),
                ],
            }).GetText());
        Assert.Equal("SELECT id FROM users", new QueryStatement(new() { Table = "users", Fields = ["id"], Joins = [] }).GetText());
        var grouped = new QueryStatement(new()
        {
            Table = "orders",
            Fields = ["status", Aggregate.Count("*", "n"), Aggregate.Sum("amount", "total")],
            Where = new WhereClause(new Operator.Equals("status", "open")),
            Joins = [Join.Inner("customers", "orders.customer_id", "customers.id")],
            GroupBy = ["status"],
            OrderBy = [OrderBy.Desc("total"), OrderBy.Asc("status")],
            Limit = 10,
        });
        Assert.Equal("SELECT status, COUNT(*) AS n, SUM(amount) AS total FROM orders INNER JOIN customers ON orders.customer_id = customers.id WHERE status = $status GROUP BY status ORDER BY total DESC, status ASC LIMIT 10", grouped.GetText());
        var byStatus = new QueryStatement(new() { Table = "users", Fields = ["id"], GroupBy = ["id"] });
        Assert.Equal("SELECT id FROM users GROUP BY id", byStatus.GetText());
        var bounds = new QueryStatement(new() { Table = "users", Fields = [Aggregate.Min("n"), Aggregate.Max("n")] });
        Assert.Equal("SELECT MIN(n), MAX(n) FROM users", bounds.GetText());
        var ordered = new QueryStatement(new() { Table = "users", Fields = ["id"], OrderBy = [OrderBy.Asc("id"), OrderBy.Desc("name")] });
        Assert.Equal("SELECT id FROM users ORDER BY id ASC, name DESC", ordered.GetText());
        var limited = new QueryStatement(new() { Table = "users", Fields = ["id"], Limit = 10 });
        Assert.Equal("SELECT id FROM users LIMIT 10", limited.GetText());
        Assert.Equal("SELECT id FROM users LIMIT 0", new QueryStatement(new() { Table = "users", Fields = ["id"], Limit = 0 }).GetText());
        Assert.Throws<InvalidOperationException>(() => new QueryStatement(new() { Table = "users", Fields = ["id"], Limit = -1 }));
        var insert = new InsertStatement("users", fields);
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", insert.GetText());
        var update = new UpdateStatement("users", fields, where);
        Assert.Equal("UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age", update.GetText());
        Assert.Same(where, update.Where);
        Assert.Null(new UpdateStatement("users", fields).Where);
        Assert.Equal("UPDATE users SET id = $id, name = $name", new UpdateStatement("users", fields).GetText());
        var delete = new DeleteStatement("users", where);
        Assert.Equal("DELETE FROM users WHERE id = $id AND age > $age", delete.GetText());
        Assert.Same(where, delete.Where);
        Assert.Equal("DELETE FROM users", new DeleteStatement("users").GetText());
        var createTable = new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("name", "TEXT")]);
        Assert.Equal("CREATE TABLE users (id INTEGER, name TEXT)", createTable.GetText());
        var namedTable = new CreateTableStatement(
            "orders",
            [
                new Column("id", "INTEGER", new ColumnDefault("0", "df_id"), new NotNull("nn_id")),
                new Column("tenant_id", "INTEGER"),
                new Column("user_id", "INTEGER"),
                new Column("qty", "INTEGER")
            ],
            [
                new PrimaryKey("pk_orders", ["id", "tenant_id"]),
                new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade),
                new UniqueConstraint("uq_orders_user", ["user_id"]),
                new CheckConstraint("chk_orders_qty", "qty > 0")
            ]);
        Assert.Equal(
            "CREATE TABLE orders (id INTEGER CONSTRAINT df_id DEFAULT 0 CONSTRAINT nn_id NOT NULL, tenant_id INTEGER, user_id INTEGER, qty INTEGER, CONSTRAINT pk_orders PRIMARY KEY (id, tenant_id), CONSTRAINT fk_orders_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE, CONSTRAINT uq_orders_user UNIQUE (user_id), CONSTRAINT chk_orders_qty CHECK (qty > 0))",
            namedTable.GetText());
        Assert.Equal(
            "CREATE TABLE items (id INTEGER DEFAULT 0, name TEXT NOT NULL, CONSTRAINT fk_items_user FOREIGN KEY (id) REFERENCES users (id))",
            new CreateTableStatement(
                "items",
                [new Column("id", "INTEGER", new ColumnDefault("0")), new Column("name", "TEXT", NotNull: new NotNull())],
                [new ForeignKey("fk_items_user", ["id"], "users", ["id"])]).GetText());
        Assert.Equal(
            "CREATE TABLE users (id INTEGER)",
            new CreateTableStatement("users", [new Column("id", "INTEGER")], []).GetText());
        Assert.Equal(
            "CREATE TABLE t (id INTEGER, CONSTRAINT fk_set_null FOREIGN KEY (id) REFERENCES u (id) ON DELETE SET NULL ON UPDATE SET DEFAULT)",
            new CreateTableStatement(
                "t",
                [new Column("id", "INTEGER")],
                [new ForeignKey("fk_set_null", ["id"], "u", ["id"], ReferentialAction.SetNull, ReferentialAction.SetDefault)]).GetText());
        Assert.Equal(
            "CREATE TABLE t (id INTEGER, CONSTRAINT fk_restrict FOREIGN KEY (id) REFERENCES u (id) ON DELETE RESTRICT ON UPDATE NO ACTION)",
            new CreateTableStatement(
                "t",
                [new Column("id", "INTEGER")],
                [new ForeignKey("fk_restrict", ["id"], "u", ["id"], ReferentialAction.Restrict, ReferentialAction.NoAction)]).GetText());
        Assert.Throws<InvalidOperationException>(() =>
            new CreateTableStatement("t", [new Column("id", "INTEGER")], [new EmptyTableConstraint()]).GetText());
        Assert.Throws<InvalidOperationException>(() =>
            new CreateTableStatement(
                "t",
                [new Column("id", "INTEGER")],
                [new ForeignKey("fk", ["id"], "u", ["id"], (ReferentialAction)99)]).GetText());
        var createIndex = new CreateIndexStatement("idx_orders_user", "orders", ["user_id"]);
        Assert.Equal("CREATE INDEX idx_orders_user ON orders (user_id)", createIndex.GetText());
        Assert.Equal(
            "CREATE UNIQUE INDEX uq_orders_code ON orders (code)",
            new CreateIndexStatement("uq_orders_code", "orders", ["code"], true).GetText());
        var call = new ExecuteStoredProcedureStatement("save", fields);
        Assert.Equal("CALL save($id, $name)", call.GetText());
        Assert.Equal(fields, call.Parameters);
        Assert.Equal("CALL save()", new ExecuteStoredProcedureStatement("save", []).GetText());
        var ret = new ReturnStatement("add", fields);
        Assert.Equal("SELECT add($id, $name)", ret.GetText());
        Assert.Equal("SELECT add()", new ReturnStatement("add", []).GetText());
        Assert.Equal(
            "CALL save($id)",
            new ExecuteStoredProcedureStatement("save", [new Parameter("id", 1), new Parameter("ret", ParameterDirection.ReturnValue)]).GetText());
        Assert.Equal(
            "SELECT add($id)",
            new ReturnStatement("add", [new Parameter("id", 1), new Parameter("ret", ParameterDirection.ReturnValue)]).GetText());
        Assert.Equal(ParameterDirection.Input, new Parameter("id", 1).Direction);
        Assert.Equal(ParameterDirection.Output, new Parameter("count", ParameterDirection.Output).Direction);
        Assert.Equal(Value.Null.Instance, new Parameter("count", ParameterDirection.Output).Value);
        Assert.Null(new Response(1, 2).Parameters);
        Assert.Equal("INSERT INTO t () VALUES ()", new InsertStatement("INSERT INTO t () VALUES ()").GetText());
        Assert.Equal("x", new UpdateStatement("x").GetText());
        Assert.Equal("x", DeleteStatement.Raw("x").GetText());
        Assert.Equal("x", new CreateTableStatement("x").GetText());
        Assert.Equal("x", new CreateIndexStatement("x").GetText());
        Assert.Equal("x", new ExecuteStoredProcedureStatement("x").GetText());
        Assert.Empty(new ExecuteStoredProcedureStatement("x").Parameters);
        Assert.Equal("x", new ReturnStatement("x").GetText());
    }

    [Fact]
    public void EvaluatesWhereClause()
    {
        var evaluator = new StatementEvaluator();
        IReadOnlyDictionary<string, Value> row = new Dictionary<string, Value> { ["id"] = 1, ["age"] = 21, ["name"] = "a" };
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.Equals("id", 1)), row));
        Assert.Equal(new Value.Bool(false), evaluator.Evaluate(new WhereClause(new Operator.Equals("id", 2)), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.NotEquals("id", 2)), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.GreaterThan("age", 18)), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.LessThan("age", 30)), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.GreaterThanOrEqual("age", 21)), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.LessThanOrEqual("age", 21)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new WhereClause(new Operator.And(
                new Operator.Equals("id", 1),
                new Operator.GreaterThan("age", 18))), row));
        Assert.Equal(
            new Value.Bool(false),
            evaluator.Evaluate(new WhereClause(new Operator.And(
                new Operator.Equals("id", 0),
                new Operator.GreaterThan("age", 18))), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new WhereClause(new Operator.Or(
                new Operator.Equals("id", 0),
                new Operator.Equals("name", "a"))), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new WhereClause(new Operator.Or(
                new Operator.Equals("id", 1),
                new Operator.Equals("name", "z"))), row));
        Assert.Equal(
            new Value.Bool(false),
            evaluator.Evaluate(new WhereClause(new Operator.Not(new Operator.Equals("id", 1))), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.In("id", [1, 2])), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new WhereClause(new Operator.Equals(new Literal(1), new Literal(1))), row));
        Assert.Equal(
            new Value.Bool(false),
            evaluator.Evaluate(new WhereClause(new Operator.Equals(new Literal(1), new Literal(0))), row));
        Assert.Throws<InvalidOperationException>(() => evaluator.Evaluate(new Where("id = 1"), row));
        Assert.Throws<InvalidOperationException>(() => evaluator.Evaluate(new ArrayExpression([]), row));
        Assert.Throws<InvalidOperationException>(() => evaluator.Evaluate(new Where("x"), row));
        Assert.Throws<InvalidOperationException>(() =>
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(true), new Literal(1)), row));
        Assert.Throws<InvalidOperationException>(() =>
            evaluator.Evaluate(new Operator.In(new Identifier("id"), new Literal(1)), row));
        Assert.Equal(Value.Null.Instance, evaluator.Evaluate(new Identifier("missing"), row));
        Assert.Equal(Value.Null.Instance, evaluator.Evaluate(new Identifier("id"), null));
        Assert.Equal(Value.Null.Instance, evaluator.Evaluate(new Literal(Value.Null.Instance), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.LessThan("name", "b")), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.GreaterThanOrEqual("name", "a")), row));
        Assert.Equal(new Value.Bool(true), evaluator.Evaluate(new WhereClause(new Operator.GreaterThan("name", "Z")), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2.5), new Literal(1.5)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2.5m), new Literal(1.5m)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2), new Literal(1.5)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2.5), new Literal(1)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2), new Literal(1.5m)), row));
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(new Operator.GreaterThan(new Literal(2.5m), new Literal(1)), row));
        var now = new DateTime(2024, 1, 2, 0, 0, 0, DateTimeKind.Utc);
        Assert.Equal(
            new Value.Bool(true),
            evaluator.Evaluate(
                new Operator.GreaterThan(new Literal(now.AddDays(1)), new Literal(now)),
                row));
    }

    [Fact]
    public async Task Readers()
    {
        var empty = new ArrayReader(Array.Empty<object?>());
        Assert.Null(empty.Current);
        Assert.False(await empty.Next());
        var reader = new ArrayReader(new object?[] { "a", "b" });
        Assert.True(await reader.Next());
        Assert.Equal("a", reader.Current);
        Assert.True(await reader.Next());
        Assert.False(await reader.Next());
        var typed = new ArrayReader<string>(new[] { "x", "y" });
        Assert.True(await typed.Next());
        Assert.Equal("x", typed.Current);
        Assert.True(await typed.Next());
        Assert.False(await typed.Next());
    }

    [Fact]
    public async Task Middleware()
    {
        var host = new RecordingCommand();
        Assert.Equal("ok", await host.Run("", null, (_, _) => Task.FromResult(("ok", EmptyRows()))));

        var seen = new List<string>();
        string? used = null;
        host.AddMiddleware(new DelegateMiddleware(
            (query, _) =>
            {
                seen.Add($"before:{query}");
                return Task.FromResult<(string, IReadOnlyList<Parameter>?)?>(("SELECT 2", null));
            },
            (query, _, results, _, _) =>
            {
                seen.Add($"after:{query}:{results.Count}");
                return Task.CompletedTask;
            }));
        await host.Run("SELECT 1", null, (query, _) =>
        {
            used = query;
            seen.Add("op");
            return Task.FromResult((7, Rows(new Row { ["id"] = 1 })));
        });
        Assert.Equal("SELECT 2", used);
        Assert.Equal(new[] { "before:SELECT 1", "op", "after:SELECT 2:1" }, seen);

        var blocked = new RecordingCommand();
        var ran = false;
        blocked.AddMiddleware(new DelegateMiddleware(
            (_, _) => throw new InvalidOperationException("blocked"),
            (_, _, _, _, _) => { ran = true; return Task.CompletedTask; }));
        await Assert.ThrowsAsync<InvalidOperationException>(() => blocked.Run("select", null, (_, _) =>
        {
            ran = true;
            return Task.FromResult((1, EmptyRows()));
        }));
        Assert.False(ran);

        Exception? captured = null;
        var failed = new RecordingCommand();
        failed.AddMiddleware(new DelegateMiddleware(
            (_, _) => Task.FromResult<(string, IReadOnlyList<Parameter>?)?>(null),
            (_, _, _, _, error) => { captured = error; return Task.CompletedTask; }));
        await Assert.ThrowsAsync<InvalidOperationException>(() => failed.Run<int>("select", null, (_, _) => throw new InvalidOperationException("fail-op")));
        Assert.Equal("fail-op", captured?.Message);
    }

    [Fact]
    public void ConsoleMiddlewareFormat()
    {
        Assert.Equal("2ms", ConsoleCommandMiddleware.FormatElapsed(2));
        Assert.Equal("1ms", ConsoleCommandMiddleware.FormatElapsed(1));
        Assert.Equal("850μs", ConsoleCommandMiddleware.FormatElapsed(0.85));
        Assert.Equal("400μs", ConsoleCommandMiddleware.FormatElapsed(0.4));
    }

    [Fact]
    public async Task ConsoleMiddlewareLines()
    {
        var lines = new List<string>();
        var stamps = new Queue<DateTimeOffset>(new[]
        {
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 101, TimeSpan.Zero),
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 103, TimeSpan.Zero),
        });
        var host = new RecordingCommand();
        host.AddMiddleware(new ConsoleCommandMiddleware(lines.Add, () => stamps.Dequeue()));
        await host.Run("SELECT * FROM \"widget\" ORDER BY \"id\"", null, (_, _) => Task.FromResult((1, EmptyRows())));
        Assert.Equal("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT * FROM \"widget\" ORDER BY \"id\"]", lines[0]);
        Assert.StartsWith("[2026-08-28T01:54:12.103Z]-[datasource][Finish]-[SELECT * FROM \"widget\" ORDER BY \"id\"] ", lines[1]);
        new ConsoleCommandMiddleware();
    }

    [Fact]
    public async Task ConsoleMiddlewareTrimsQuery()
    {
        var lines = new List<string>();
        var host = new RecordingCommand();
        host.AddMiddleware(new ConsoleCommandMiddleware(lines.Add));
        await host.Run("SELECT 1\n", null, (_, _) => Task.FromResult((1, EmptyRows())));
        var signature = System.Text.RegularExpressions.Regex.Replace(lines[0], @"^\[[^\]]+\]-", "");
        Assert.Equal("[datasource][Start]-[SELECT 1]", signature);
    }

    [Fact]
    public async Task ConsoleMiddlewareVerboseBodies()
    {
        var lines = new List<string>();
        var stamps = new Queue<DateTimeOffset>(new[]
        {
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 101, TimeSpan.Zero),
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 102, TimeSpan.Zero),
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 103, TimeSpan.Zero),
            new DateTimeOffset(2026, 8, 28, 1, 54, 12, 104, TimeSpan.Zero),
        });
        var host = new RecordingCommand();
        host.AddMiddleware(new ConsoleCommandMiddleware(lines.Add, () => stamps.Count > 0
            ? stamps.Dequeue()
            : new DateTimeOffset(2026, 8, 28, 1, 54, 12, 104, TimeSpan.Zero), new Dictionary<string, string>
        {
            ["isVerbose"] = "true"
        }));
        await host.Run(
            "SELECT 1",
            [new Parameter("id", 1L)],
            (_, _) => Task.FromResult((1, Rows(new Row { ["id"] = 1L }))));
        Assert.Equal(4, lines.Count);
        Assert.Equal("[2026-08-28T01:54:12.101Z]-[datasource][Start]-[SELECT 1]", lines[0]);
        Assert.Equal("[2026-08-28T01:54:12.102Z]-[datasource][Request]-[{id: 1}]", lines[1]);
        Assert.StartsWith("[2026-08-28T01:54:12.103Z]-[datasource][Finish]-[SELECT 1] ", lines[2]);
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{id: 1}]", lines[3]);
        lines.Clear();
        await host.Run(
            "DELETE FROM user",
            [new Parameter("id", 2L)],
            (_, _) => Task.FromResult((new Response(Value.Null.Instance, 1), EmptyRows())));
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{result: null, rowsAffected: 1}]", lines[3]);
        lines.Clear();
        await host.Run(
            "SELECT 1",
            [new Parameter("name", "Ada"), new Parameter("missing", Value.Null.Instance), new Parameter("on", true), new Parameter("off", false)],
            (_, _) => Task.FromResult((1, Rows(new Row { ["name"] = "Ada", ["missing"] = Value.Null.Instance, ["on"] = true, ["off"] = false }))));
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][Request]-[{name: Ada, missing: null, on: True, off: False}]", lines[1]);
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][AfterCommand]-[{missing: null, name: Ada, off: False, on: True}]", lines[3]);
    }

    [Fact]
    public void Registry()
    {
        Assert.Empty(DatasourceFactory.Registered());
        DatasourceFactory.Register("json", new MockDriver());
        DatasourceFactory.Register("memory", new MockDriver());
        Assert.Equal(new[] { "json", "memory" }, DatasourceFactory.Registered());
        Assert.Equal("json", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "json" }).Provider);
        DatasourceFactory.Reset();
        Assert.Throws<ProviderNotInstalledException>(() =>
            DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "json" }));
    }

    [Fact]
    public async Task Defaults()
    {
        Assert.Throws<DatasourceException>(() => DatasourceFactory.Create());
        DatasourceFactory.Register("sqlite", new MockDriver());
        var ds = DatasourceFactory.Create(new Dictionary<string, string?>
        {
            ["provider"] = "sqlite",
            ["database_url"] = "sqlite:///:memory:"
        });
        Assert.Equal("sqlite", ds.Provider);
        var connection = await ds.CreateConnection();
        Assert.False(connection.IsOpen);
        Assert.Same(connection, ds.CreateCommand().Connection);
        Assert.IsType<CRUDRepository<object>>(ds.CreateRepository<object>());
        Assert.IsType<ReadonlyRepository<object>>(ds.CreateReadonlyRepository<object>());
        DatasourceFactory.Reset();
        var error = Assert.Throws<DatasourceException>(() => DatasourceFactory.Create());
        Assert.Equal("Datasource defaults are not specified", error.Message);
        Assert.Throws<DatasourceException>(() => _ = DatasourceFactory.Default);
        DatasourceFactory.Register("sqlite", new MockDriver());
        DatasourceFactory.Register("memory", new MockDriver());
        Environment.SetEnvironmentVariable("DATASOURCE_URL", "sqlite:///:memory:");
        Assert.Equal("sqlite", DatasourceFactory.Default.Provider);
        Environment.SetEnvironmentVariable("DATASOURCE_URL", null);
        Environment.SetEnvironmentVariable("PROVIDER", "sqlite");
        Assert.Equal("sqlite", DatasourceFactory.Default.Provider);
        Environment.SetEnvironmentVariable("DEFAULT_PROVIDER", "memory");
        Environment.SetEnvironmentVariable("DEFAULT_DATASOURCE_URL", "memory://");
        Assert.Equal("memory", DatasourceFactory.Default.Provider);
    }

    [Fact]
    public async Task Stubs()
    {
        var ds = StubFactory.Create("text");
        Assert.Equal("text", ds.Provider);
        var connection = await ds.CreateConnection();
        Assert.False(connection.IsOpen);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.Open());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.Close());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.StartTransaction());
        var command = ds.CreateCommand();
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute(""));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader("select", Array.Empty<Parameter>()));
        var provided = new StubConnection("text");
        Assert.Same(provided, ds.CreateCommand(provided).Connection);
        var repo = ds.CreateRepository<object>(provided);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.ReadSingle());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.ReadSingle(new Where("1=1")));
        Assert.Throws<ProviderNotImplementedException>(() => repo.Read());
        Assert.Throws<ProviderNotImplementedException>(() => repo.Read(new Where("1=1")));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Create(new object()));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Update(new object()));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Delete(new object(), new Where("1=1")));
        Assert.IsType<StubRepository<object>>(ds.CreateRepository<object>());
        Assert.IsType<StubRepository<object>>(((IDatasource)ds).CreateReadonlyRepository<object>(provided));
        var txn = new StubTransaction("text", provided);
        Assert.Same(provided, txn.Connection);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => txn.Commit());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => txn.Abort());
        var reader = new StubReader("text");
        Assert.Null(reader.Current);
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => reader.Next());
    }

    [Fact]
    public async Task Mocks()
    {
        var connection = new MockConnection();
        await connection.Open();
        Assert.True(connection.IsOpen);
        var started = await connection.StartTransaction(IsolationLevel.ReadUncommitted);
        Assert.Equal(IsolationLevel.ReadUncommitted, started.Isolation);
        await connection.Close();
        connection.OpenError = new InvalidOperationException("open-fail");
        await Assert.ThrowsAsync<InvalidOperationException>(() => connection.Open());
        connection.CloseError = new InvalidOperationException("close-fail");
        await Assert.ThrowsAsync<InvalidOperationException>(() => connection.Close());
        connection.StartTransactionError = new InvalidOperationException("tx-fail");
        await Assert.ThrowsAsync<InvalidOperationException>(() => connection.StartTransaction());
        var custom = new MockTransaction(connection);
        Assert.Same(custom, new MockConnection(custom).Transaction);

        var txn = new MockTransaction(connection);
        Assert.Same(connection, txn.Connection);
        await txn.Commit();
        Assert.True(txn.Committed);
        await txn.Abort();
        Assert.True(txn.Aborted);
        txn.CommitError = new InvalidOperationException("c");
        await Assert.ThrowsAsync<InvalidOperationException>(() => txn.Commit());
        txn.AbortError = new InvalidOperationException("a");
        await Assert.ThrowsAsync<InvalidOperationException>(() => txn.Abort());

        var command = new MockCommand(connection) { Response = new Response(1, 2), Reader = new MockReader<Row>([new Row { ["id"] = 1 }]) };
        Assert.Equal(2, (await command.Execute("insert", new[] { new Parameter("id", 1) })).RowsAffected);
        await command.ExecuteReader("");
        command.ExecuteError = new InvalidOperationException("exec");
        await Assert.ThrowsAsync<InvalidOperationException>(() => command.Execute(""));
        command.ExecuteReaderError = new InvalidOperationException("read");
        await Assert.ThrowsAsync<InvalidOperationException>(() => command.ExecuteReader("x"));

        command.ExecuteError = null;
        command.ExecuteReaderError = null;
        Parameter[] fields = [new("id", 1)];
        var statement = new ExecuteStoredProcedureStatement("save", fields);
        ICommand asCommand = command;
        await asCommand.Execute(statement, fields);
        Assert.Equal("CALL save($id)", command.ExecuteCalls[^1].Text);
        Assert.Same(fields, command.ExecuteCalls[^1].Parameters);
        await asCommand.ExecuteReader(statement, fields);
        Assert.Equal("CALL save($id)", command.ExecuteReaderCalls[^1].Text);
        Assert.Same(fields, command.ExecuteReaderCalls[^1].Parameters);

        var reader = new MockReader(new object?[] { "x" });
        Assert.True(await reader.Next());
        Assert.False(await reader.Next());
        reader.NextError = new InvalidOperationException("next");
        await Assert.ThrowsAsync<InvalidOperationException>(() => reader.Next());

        var repo = new MockRepository<Dictionary<string, Value>>(connection);
        Assert.Null(await repo.ReadSingle());
        repo.Items.Add(new Row { ["id"] = 1 });
        Assert.NotNull(await repo.ReadSingle(new MockQuery("id=1")));
        Assert.NotNull(await repo.ReadSingle(new MockWhere("id=1")));
        Assert.Single(await repo.Read().ToListAsync());
        Assert.Single(await repo.Read(new MockQuery("all")).ToListAsync());
        Assert.Single(await repo.Read(new MockWhere("all")).ToListAsync());
        await repo.Create(new Row { ["id"] = 3 });
        await repo.Update(new Row { ["id"] = 3 }, new MockWhere("id=3"));
        await repo.Delete(new Row { ["id"] = 3 });
        repo.ReadSingleError = new InvalidOperationException("gs");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.ReadSingle());
        repo.ReadError = new InvalidOperationException("g");
        await Assert.ThrowsAsync<InvalidOperationException>(async () => await repo.Read().ToListAsync());
        repo.CreateError = new InvalidOperationException("ad");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Create(new Row()));
        repo.UpdateError = new InvalidOperationException("up");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Update(new Row()));
        repo.DeleteError = new InvalidOperationException("del");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Delete(new Row()));

        var ds = new MockDatasource();
        Assert.Equal("mock", ds.Provider);
        await ds.CreateConnection();
        var mockCommand = Assert.IsType<MockCommand>(ds.CreateCommand());
        await mockCommand.Execute("");
        await mockCommand.ExecuteReader("");
        var provided = new MockConnection();
        Assert.Same(provided, ds.CreateCommand(provided).Connection);
        Assert.IsType<MockRepository<object>>(ds.CreateRepository<object>());
        Assert.IsType<MockRepository<object>>(ds.CreateRepository<object>(provided));
        ds.Command = new MockCommand(ds.Connection);
        ds.Repository = new MockRepository<object>(ds.Connection);
        Assert.Same(ds.Command, ds.CreateCommand());
        Assert.Same(ds.Repository, ds.CreateRepository<object>());
        Assert.Same(ds.Repository, ((IDatasource)ds).CreateReadonlyRepository<object>());
        Assert.Equal("custom", new MockDatasource("custom").Provider);

        var q = new MockQuery("q");
        var w = new MockWhere("w");
        Assert.Equal("q", q.GetText());
        Assert.Equal("w", w.GetText());
        Assert.Equal(1, q.GetTextCalls);
        Assert.Equal(1, w.GetTextCalls);

        var seen = new List<string>();
        command.AddMiddleware(new DelegateMiddleware(
            (query, _) => { seen.Add($"before:{query}"); return Task.FromResult<(string, IReadOnlyList<Parameter>?)?>(null); },
            (query, _, _, _, _) => { seen.Add($"after:{query}"); return Task.CompletedTask; }));
        command.ExecuteError = null;
        await command.Execute("insert");
        Assert.Equal(new[] { "before:insert", "after:insert" }, seen);
    }

    private static IReadOnlyList<Row> EmptyRows() => [];

    private static IReadOnlyList<Row> Rows(params Row[] rows) => rows;

    private sealed class RecordingCommand : CommandMiddlewareHost, ICommand
    {
        public IConnection Connection { get; } = new MockConnection();
        public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
            throw new NotSupportedException();
        public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
            throw new NotSupportedException();
    }

    private sealed class DelegateMiddleware : ICommandMiddleware
    {
        private readonly Func<string, IReadOnlyList<Parameter>?, Task<(string Text, IReadOnlyList<Parameter>? Parameters)?>> _before;
        private readonly Func<string, IReadOnlyList<Parameter>?, IReadOnlyList<Row>, double, Exception?, Task> _after;

        public DelegateMiddleware(
            Func<string, IReadOnlyList<Parameter>?, Task<(string Text, IReadOnlyList<Parameter>? Parameters)?>> before,
            Func<string, IReadOnlyList<Parameter>?, IReadOnlyList<Row>, double, Exception?, Task> after)
        {
            _before = before;
            _after = after;
        }

        public Task<(string Text, IReadOnlyList<Parameter>? Parameters)?> BeforeCommand(string text, IReadOnlyList<Parameter>? parameters) =>
            _before(text, parameters);

        public Task AfterCommand(
            string text,
            IReadOnlyList<Parameter>? parameters,
            IReadOnlyList<Row> results,
            double elapsedMs,
            Exception? error = null) =>
            _after(text, parameters, results, elapsedMs, error);
    }

    sealed record EmptyTableConstraint : ITableConstraint;
}
