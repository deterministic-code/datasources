using Datasources;
using Datasources.Builders;

namespace Datasources.Tests;

public class CoreTests
{
    public CoreTests() => Datasource.Reset();

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
        Assert.Equal("SELECT 1", new Query("SELECT 1").GetText());
        Assert.Null(new Query("SELECT 1").Where);
        var byId = new Where("id = 1");
        Assert.Same(byId, new Query("SELECT 1", byId).Where);
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
        var where = WhereBuilder.And(WhereBuilder.Equals("id", 1), WhereBuilder.GreaterThan("age", 18));
        Assert.Equal("id = $id AND age > $age", where.GetText());
        Assert.Equal(2, where.Parameters.Count);
        Assert.Equal(new Value.Int(1), where.Parameters[0].Value);
        Assert.Equal(new Value.Int(18), where.Parameters[1].Value);
        Assert.Equal("id <> $id", WhereBuilder.Ne("id", 2).GetText());
        Assert.Equal("n < $n", WhereBuilder.Lt("n", 0).GetText());
        Assert.Equal("n >= $n", WhereBuilder.Gte("n", 0).GetText());
        Assert.Equal("n <= $n", WhereBuilder.Lte("n", 0).GetText());
        Assert.Equal("1 = 1", WhereBuilder.And().GetText());
        Assert.Equal("1 = 0", WhereBuilder.Or().GetText());
        Assert.Equal("id = $id", WhereBuilder.And(WhereBuilder.Equals("id", 1)).GetText());
        Assert.Equal("id = $id", WhereBuilder.Or(WhereBuilder.Equals("id", 1)).GetText());
        Assert.Equal(
            "(id = $id AND age > $age) OR name = $name",
            WhereBuilder.Or(where, WhereBuilder.Equals("name", "a")).GetText());
        Assert.Equal("NOT (id = $id)", WhereBuilder.Not(WhereBuilder.Equals("id", 1)).GetText());
        Assert.Equal("1 = 0", WhereBuilder.In("id", []).GetText());
        Assert.Empty(WhereBuilder.In("id", []).Parameters);
        var inIds = WhereBuilder.In("id", [1, 2]);
        Assert.Equal("id IN ($id_0, $id_1)", inIds.GetText());
        Assert.Equal(["id_0", "id_1"], inIds.Parameters.Select(parameter => parameter.Name));
        Assert.Equal(new Value.Int(1), inIds.Parameters[0].Value);
        Assert.Equal(new Value.Int(2), inIds.Parameters[1].Value);

        var fields = new[] { new Parameter("id", 1), new Parameter("name", "a") };
        var query = StatementBuilder.CreateQuery("users", ["id", "name"], where);
        Assert.Equal("SELECT id, name FROM users WHERE id = $id AND age > $age", query.GetText());
        Assert.Same(where, query.Where);
        Assert.Equal("SELECT * FROM users", StatementBuilder.CreateQuery("users", []).GetText());
        Assert.Null(StatementBuilder.CreateQuery("users", []).Where);
        Assert.Equal("SELECT id, name FROM users", StatementBuilder.CreateQuery("users", ["id", "name"]).GetText());
        Assert.Equal(
            "SELECT users.id, orders.total FROM users INNER JOIN orders ON users.id = orders.user_id",
            StatementBuilder.CreateQuery("users", ["users.id", "orders.total"], null, [
                Join.Inner("orders", "users.id", "orders.user_id"),
            ]).GetText());
        Assert.Equal(
            "SELECT users.id FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN payments ON orders.id = payments.order_id RIGHT JOIN profiles ON users.id = profiles.user_id WHERE users.id = $users.id",
            StatementBuilder.CreateQuery(
                "users",
                ["users.id"],
                WhereBuilder.Equals("users.id", 1),
                [
                    Join.Inner("orders", "users.id", "orders.user_id"),
                    Join.Left("payments", "orders.id", "payments.order_id"),
                    Join.Right("profiles", "users.id", "profiles.user_id"),
                ]).GetText());
        Assert.Equal("SELECT id FROM users", StatementBuilder.CreateQuery("users", ["id"], null, []).GetText());
        var insert = StatementBuilder.CreateInsert("users", fields);
        Assert.Equal("INSERT INTO users (id, name) VALUES ($id, $name)", insert.GetText());
        var update = StatementBuilder.CreateUpdate("users", fields, where);
        Assert.Equal("UPDATE users SET id = $id, name = $name WHERE id = $id AND age > $age", update.GetText());
        Assert.Same(where, update.Where);
        Assert.Null(StatementBuilder.CreateUpdate("users", fields).Where);
        Assert.Equal("UPDATE users SET id = $id, name = $name", StatementBuilder.CreateUpdate("users", fields).GetText());
        var delete = StatementBuilder.CreateDelete("users", where);
        Assert.Equal("DELETE FROM users WHERE id = $id AND age > $age", delete.GetText());
        Assert.Same(where, delete.Where);
        Assert.Equal("DELETE FROM users", StatementBuilder.CreateDelete("users").GetText());
        Assert.Equal(
            "CREATE TABLE users (id INTEGER, name TEXT)",
            StatementBuilder.CreateTable("users", [new Column("id", "INTEGER"), new Column("name", "TEXT")]).GetText());
        Assert.Equal("CALL save($id, $name)", StatementBuilder.CreateStoredProcedure("save", fields).GetText());
        Assert.Equal("CALL save()", StatementBuilder.CreateStoredProcedure("save").GetText());
        Assert.Equal("INSERT INTO t () VALUES ()", new InsertStatement("INSERT INTO t () VALUES ()").GetText());
        Assert.Equal("x", new UpdateStatement("x").GetText());
        Assert.Equal("x", new DeleteStatement("x").GetText());
        Assert.Equal("x", new CreateTableStatement("x").GetText());
        Assert.Equal("x", new StoredProcedureStatement("x").GetText());
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
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][Response]-[{id: 1}]", lines[3]);
        lines.Clear();
        await host.Run(
            "DELETE FROM user",
            [new Parameter("id", 2L)],
            (_, _) => Task.FromResult((new Response(Value.Null.Instance, 1), EmptyRows())));
        Assert.Equal("[2026-08-28T01:54:12.104Z]-[datasource][Response]-[{result: null, rowsAffected: 1}]", lines[3]);
    }

    [Fact]
    public void Registry()
    {
        Assert.Empty(Datasource.Registered());
        Datasource.Register(new MockDatasource("json"), new MockDatasource("memory"));
        Assert.Equal(new[] { "json", "memory" }, Datasource.Registered());
        Assert.Equal("json", Datasource.Create("json").Provider);
        Datasource.Reset();
        Assert.Throws<ProviderNotInstalledException>(() => Datasource.Create("json"));
    }

    [Fact]
    public async Task Defaults()
    {
        Assert.Throws<DatasourceException>(() => Datasource.Create());
        Datasource.Register(new MockDatasource("sqlite"));
        Datasource.Defaults(new Dictionary<string, string?>
        {
            ["provider"] = "sqlite",
            ["filename"] = ":memory:"
        });
        var ds = Datasource.Create();
        Assert.Equal("sqlite", ds.Provider);
        var connection = await ds.CreateConnection();
        Assert.False(connection.IsOpen);
        Assert.Same(connection, ds.CreateCommand().Connection);
        Assert.IsType<MockRepository<object>>(ds.CreateRepository<object>());
        Datasource.Reset();
        var error = Assert.Throws<DatasourceException>(() => Datasource.Create());
        Assert.Equal("Datasource defaults are not specified", error.Message);
    }

    [Fact]
    public async Task Stubs()
    {
        var ds = StubFactory.Create("text");
        Assert.Equal("text", ds.Provider);
        Assert.Equal("text", ds.Create().Provider);
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
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.GetSingle());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.GetSingle(new Where("1=1")));
        Assert.Throws<ProviderNotImplementedException>(() => repo.Get());
        Assert.Throws<ProviderNotImplementedException>(() => repo.Get(new Where("1=1")));
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
        var statement = StatementBuilder.CreateStoredProcedure("save", fields);
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
        Assert.Null(await repo.GetSingle());
        repo.Items.Add(new Row { ["id"] = 1 });
        Assert.NotNull(await repo.GetSingle(new MockQuery("id=1")));
        Assert.NotNull(await repo.GetSingle(new MockWhere("id=1")));
        Assert.Single(await repo.Get().ToListAsync());
        Assert.Single(await repo.Get(new MockQuery("all")).ToListAsync());
        Assert.Single(await repo.Get(new MockWhere("all")).ToListAsync());
        await repo.Create(new Row { ["id"] = 3 });
        await repo.Update(new Row { ["id"] = 3 }, new MockWhere("id=3"));
        await repo.Delete(new Row { ["id"] = 3 });
        repo.GetSingleError = new InvalidOperationException("gs");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.GetSingle());
        repo.GetError = new InvalidOperationException("g");
        await Assert.ThrowsAsync<InvalidOperationException>(async () => await repo.Get().ToListAsync());
        repo.CreateError = new InvalidOperationException("ad");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Create(new Row()));
        repo.UpdateError = new InvalidOperationException("up");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Update(new Row()));
        repo.DeleteError = new InvalidOperationException("del");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Delete(new Row()));

        var ds = new MockDatasource();
        Assert.Equal("mock", ds.Create().Provider);
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
        Assert.Equal("custom", new MockDatasource("custom").Create().Provider);

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
        private readonly Func<string, IReadOnlyList<Parameter>?, Task<(string Query, IReadOnlyList<Parameter>? Parameters)?>> _before;
        private readonly Func<string, IReadOnlyList<Parameter>?, IReadOnlyList<Row>, double, Exception?, Task> _after;

        public DelegateMiddleware(
            Func<string, IReadOnlyList<Parameter>?, Task<(string Query, IReadOnlyList<Parameter>? Parameters)?>> before,
            Func<string, IReadOnlyList<Parameter>?, IReadOnlyList<Row>, double, Exception?, Task> after)
        {
            _before = before;
            _after = after;
        }

        public Task<(string Query, IReadOnlyList<Parameter>? Parameters)?> BeforeQuery(string query, IReadOnlyList<Parameter>? parameters) =>
            _before(query, parameters);

        public Task AfterQuery(
            string query,
            IReadOnlyList<Parameter>? parameters,
            IReadOnlyList<Row> results,
            double elapsedMs,
            Exception? error = null) =>
            _after(query, parameters, results, elapsedMs, error);
    }
}
