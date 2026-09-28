using Datasources;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

public class SqliteCommandIntegrationTests
{
    [Fact]
    public async Task ReturnStatementSelectsScalar()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = ":memory:" });
        await conn.Open();
        var command = new SqliteCommand(conn);
        var parameters = new[] { new Parameter("x", -3) };
        var response = await command.Execute(new ReturnStatement("abs", parameters).GetText(), parameters);
        Assert.Equal(new Value.Int(3), response.Result);
    }

    [Fact]
    public async Task NextResultReadsTheNextStatement()
    {
        var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = ":memory:" });
        await conn.Open();
        var command = new SqliteCommand(conn);
        var reader = await command.ExecuteReader("SELECT 1 AS id; SELECT 2 AS id");
        var first = new List<Row>();
        while (await reader.Next())
            first.Add(reader.Current!);
        var moved = await reader.NextResult();
        var second = new List<Row>();
        while (await reader.Next())
            second.Add(reader.Current!);
        var again = await reader.NextResult();
        if (reader is IAsyncDisposable disposable)
            await disposable.DisposeAsync();
        await conn.Close();
        Assert.True(moved);
        Assert.False(again);
        Assert.Equal(new Value.Int(1), first[0]["id"]);
        Assert.Equal(new Value.Int(2), second[0]["id"]);
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var dialect = SqlSpec.Dialect.Load("sqlite", static statement => new SqliteStatementFactory().Transform(statement).GetText());
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
            await SqlSpec.Run(path, dialect, target =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = target });
                return new SqlSpec.Session(conn, new SqliteCommand(conn));
            }, () => new SqliteCommand(new SqliteConnection()));
    }
}
