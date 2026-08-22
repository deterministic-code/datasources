using Datasources;

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
        Assert.Equal(1, Helpers.ParametersToRecord(new[] { new Parameter("id", 1) })["id"]);
        Assert.Equal("SELECT 1", new Query("SELECT 1").GetText());
        Assert.Equal("id = 1", new Where("id = 1").GetText());
        var withId = new Dictionary<string, object?> { ["id"] = 9, ["name"] = "a" };
        Assert.Single(Helpers.IdentityFilter(withId));
        Assert.Equal("id = $id", Helpers.IdentityWhereText(withId));
        var noId = new Dictionary<string, object?> { ["name"] = "a", ["city"] = "x" };
        Assert.Equal(2, Helpers.IdentityFilter(noId).Count);
        Assert.Contains("name = $name", Helpers.IdentityWhereText(noId));
        Assert.Equal("1 = 1", Helpers.IdentityWhereText(new Dictionary<string, object?>()));
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
        var collected = await AsyncEnumerable.Collect(AsyncEnumerable.FromReader<string>(new ArrayReader(new object?[] { "z" })));
        Assert.Equal(new[] { "z" }, collected);
        var skipped = await AsyncEnumerable.Collect(AsyncEnumerable.FromReader<string>(new ArrayReader(new object?[] { 1 })));
        Assert.Empty(skipped);
    }

    [Fact]
    public async Task Middleware()
    {
        var host = new MiddlewareHost();
        var order = new List<string>();
        host.Use();
        Assert.Equal(0, await host.Run("close", () => Task.FromResult(0)));
        host.Use(async ctx =>
        {
            order.Add($"b:{ctx.Operation}");
            await Task.CompletedTask;
        }, async ctx =>
        {
            order.Add($"a:{ctx.Operation}");
            await Task.CompletedTask;
        });
        await host.Run("open", () => Task.FromResult(7));
        Assert.Equal(new[] { "b:open", "a:open" }, order);

        var blocked = new MiddlewareHost();
        var ran = false;
        blocked.Use(_ => throw new InvalidOperationException("blocked"), _ => { ran = true; return Task.CompletedTask; });
        await Assert.ThrowsAsync<InvalidOperationException>(() => blocked.Run("execute", () =>
        {
            ran = true;
            return Task.FromResult(1);
        }));
        Assert.False(ran);

        Exception? captured = null;
        var failed = new MiddlewareHost();
        failed.Use(after: ctx =>
        {
            captured = ctx.Error;
            return Task.CompletedTask;
        });
        await Assert.ThrowsAsync<InvalidOperationException>(() => failed.Run("abort", () => throw new InvalidOperationException("fail-op")));
        Assert.Equal("fail-op", captured?.Message);

        await host.Run("execute", () => Task.CompletedTask, "insert", new[] { new Parameter("id", 1) });
    }

    [Fact]
    public void Registry()
    {
        Assert.Empty(Datasource.Registered());
        Datasource.Use(new MockDatasource("json"));
        Assert.Equal(new[] { "json" }, Datasource.Registered());
        Assert.Equal("json", Datasource.Create("json").Provider);
        Datasource.Reset();
        Assert.Throws<ProviderNotInstalledException>(() => Datasource.Create("json"));
    }

    [Fact]
    public async Task Stubs()
    {
        var ds = StubFactory.Create("text");
        Assert.Equal("text", ds.Provider);
        Assert.Equal("text", ds.Create().Provider);
        var connection = ds.CreateConnection();
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.Open());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.Close());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => connection.StartTransaction());
        var command = ds.CreateCommand();
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.Execute());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => command.ExecuteReader("select", Array.Empty<Parameter>()));
        var provided = new StubConnection("text");
        Assert.Same(provided, ds.CreateCommand(provided).Connection);
        var repo = ds.CreateRepository<object>(provided, new RepositoryOptions { Table = "items" });
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.GetSingle());
        Assert.Throws<ProviderNotImplementedException>(() => repo.Get());
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Add(new object()));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Update(new object()));
        await Assert.ThrowsAsync<ProviderNotImplementedException>(() => repo.Delete(new object(), new Where("1=1")));
        Assert.IsType<StubConnection>(ds.CreateRepository<object>().Connection);
        var txn = new StubTransaction("text", provided);
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
        Assert.True(connection.Opened);
        await connection.StartTransaction();
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
        await txn.Commit();
        Assert.True(txn.Committed);
        await txn.Abort();
        Assert.True(txn.Aborted);
        txn.CommitError = new InvalidOperationException("c");
        await Assert.ThrowsAsync<InvalidOperationException>(() => txn.Commit());
        txn.AbortError = new InvalidOperationException("a");
        await Assert.ThrowsAsync<InvalidOperationException>(() => txn.Abort());

        var command = new MockCommand(connection) { Response = new Response(1, 2), Reader = new MockReader(new object?[] { 1 }) };
        Assert.Equal(2, (await command.Execute("insert", new[] { new Parameter("id", 1) })).RowsAffected);
        await command.ExecuteReader();
        command.ExecuteError = new InvalidOperationException("exec");
        await Assert.ThrowsAsync<InvalidOperationException>(() => command.Execute());
        command.ExecuteReaderError = new InvalidOperationException("read");
        await Assert.ThrowsAsync<InvalidOperationException>(() => command.ExecuteReader("x"));

        var reader = new MockReader(new object?[] { "x" });
        Assert.True(await reader.Next());
        Assert.False(await reader.Next());
        reader.NextError = new InvalidOperationException("next");
        await Assert.ThrowsAsync<InvalidOperationException>(() => reader.Next());

        var repo = new MockRepository<Dictionary<string, object?>>(connection);
        Assert.Null(await repo.GetSingle());
        repo.Items.Add(new Dictionary<string, object?> { ["id"] = 1 });
        Assert.NotNull(await repo.GetSingle(new MockQuery("id=1")));
        Assert.Single(await AsyncEnumerable.Collect(repo.Get()));
        Assert.Single(await AsyncEnumerable.Collect(repo.Get(new MockQuery("all"))));
        await repo.Add(new Dictionary<string, object?> { ["id"] = 3 });
        await repo.Update(new Dictionary<string, object?> { ["id"] = 3 }, new MockWhere("id=3"));
        await repo.Delete(new Dictionary<string, object?> { ["id"] = 3 });
        repo.GetSingleError = new InvalidOperationException("gs");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.GetSingle());
        repo.GetError = new InvalidOperationException("g");
        await Assert.ThrowsAsync<InvalidOperationException>(() => AsyncEnumerable.Collect(repo.Get()));
        repo.AddError = new InvalidOperationException("ad");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Add(new Dictionary<string, object?>()));
        repo.UpdateError = new InvalidOperationException("up");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Update(new Dictionary<string, object?>()));
        repo.DeleteError = new InvalidOperationException("del");
        await Assert.ThrowsAsync<InvalidOperationException>(() => repo.Delete(new Dictionary<string, object?>()));

        var ds = new MockDatasource();
        Assert.Equal("mock", ds.Create().Provider);
        ds.CreateConnection();
        Assert.IsType<MockCommand>(ds.CreateCommand());
        var provided = new MockConnection();
        Assert.Same(provided, ds.CreateCommand(provided).Connection);
        Assert.IsType<MockRepository<object>>(ds.CreateRepository<object>(provided, new RepositoryOptions { Table = "t" }));
        ds.Command = new MockCommand(ds.Connection);
        ds.Repository = new MockRepository<object>(ds.Connection);
        Assert.Same(ds.Command, ds.CreateCommand());
        Assert.Same(ds.Repository, ds.CreateRepository<object>());
        Assert.Equal("custom", new MockDatasource("custom").Create().Provider);

        var q = new MockQuery("q");
        var w = new MockWhere("w");
        Assert.Equal("q", q.GetText());
        Assert.Equal("w", w.GetText());
        Assert.Equal(1, q.GetTextCalls);
        Assert.Equal(1, w.GetTextCalls);

        var seen = new List<string>();
        connection.Use(_ => { seen.Add("before"); return Task.CompletedTask; }, _ => { seen.Add("after"); return Task.CompletedTask; });
        connection.OpenError = null;
        await connection.Open();
        Assert.Equal(new[] { "before", "after" }, seen);
    }
}
