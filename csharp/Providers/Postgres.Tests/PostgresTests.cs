using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresTests
{
    public PostgresTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("postgres", new PostgresDriver());
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        var (sql, values) = PostgresBind.ToPostgres("SELECT * FROM t WHERE id = $id AND n = $id", new Row { ["id"] = 1 });
        Assert.Equal("SELECT * FROM t WHERE id = $1 AND n = $1", sql);
        Assert.Equal(new object?[] { 1L }, values);
        var stamp = new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        var (_, dates) = PostgresBind.ToPostgres("SELECT $at", new Row { ["at"] = stamp });
        Assert.Equal(new object?[] { "2020-01-02T03:04:05.000Z" }, dates);
        Assert.Contains("Host=127.0.0.1", PostgresConnectionString.ToConnectionString("postgres://ds:ds@127.0.0.1:5432/datasources"));
        Assert.Contains("Username=ds", PostgresConnectionString.ToConnectionString("postgres://ds@127.0.0.1/datasources"));
        Assert.Equal("Host=db;Port=1", PostgresConnectionString.ToConnectionString("Host=db;Port=1"));
        Assert.Equal("READ UNCOMMITTED", PostgresBind.IsolationSql(IsolationLevel.ReadUncommitted));
        Assert.Equal("READ COMMITTED", PostgresBind.IsolationSql(IsolationLevel.ReadCommitted));
        Assert.Equal("REPEATABLE READ", PostgresBind.IsolationSql(IsolationLevel.RepeatableRead));
        Assert.Equal("SERIALIZABLE", PostgresBind.IsolationSql(IsolationLevel.Serializable));

        var conn = new PostgresConnection(new PostgresConnectionOptions { Url = "postgres://db/app", Schema = "s" });
        Assert.Equal("s", conn.Schema);
        Assert.Equal("postgres://db/app", conn.Url);
        var command = new PostgresCommand(conn);
        await command.Execute("");
        await command.ExecuteReader("");

        Assert.False(conn.IsOpen);
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        var created = new PostgresConnection();
        Assert.Equal("", created.Url);
        Assert.Null(created.Schema);

        var ds = new Datasource("postgres", new PostgresDriver());
        Assert.Equal("postgres", ds.Provider);
        var provided = Assert.IsType<PostgresConnection>(await ds.CreateConnection());
        Assert.Equal("postgres", provided.Provider);
        var bound = Assert.IsType<PostgresCommand>(ds.CreateCommand(provided));
        Assert.Same(provided, bound.Connection);
        await bound.Execute("");
        await bound.ExecuteReader("");
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(
            ds.CreateRepository<Dictionary<string, Value>>(provided));
        Assert.Equal("postgres", DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "postgres" }).Provider);

        var closed = new PostgresConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new PostgresCommand(closed).Execute(Native.N("x")));
        await Assert.ThrowsAsync<DatasourceException>(() => new PostgresCommand(closed).ExecuteReader(Native.N("x")));
    }

    [Fact]
    public void MapsColumnTypes()
    {
        var driver = new PostgresDriver();
        Assert.Equal("INTEGER", driver.ColumnType("int"));
        Assert.Equal("UUID", driver.ColumnType("uuid"));
        Assert.Equal("TEXT", driver.ColumnType("string"));
        Assert.Equal("BOOLEAN", driver.ColumnType("bool"));
        Assert.Equal("NUMERIC", driver.ColumnType("decimal"));
        Assert.Equal("DOUBLE PRECISION", driver.ColumnType("float"));
        Assert.Equal("TIMESTAMPTZ", driver.ColumnType("datetime"));
        Assert.Equal("BYTEA", driver.ColumnType("binary"));
        Assert.Contains("Unknown postgres type \"xml\"", Assert.Throws<DatasourceException>(() => driver.ColumnType("xml")).Message);
    }

    [Fact]
    public void MapsPropertyTargets()
    {
        var converter = RowTypeConverter.From(new PostgresDriver().RowTypeConverters, new Dictionary<string, string>
        {
            ["Key"] = "uuid",
            ["Active"] = "bool",
            ["Login"] = "string",
            ["Created"] = "datetime",
        });
        var id = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var row = converter.Convert(new Row
        {
            ["Key"] = id.ToString(),
            ["Active"] = 1L,
            ["Login"] = "ada",
            ["Created"] = "2020-01-02T03:04:05.000Z",
        });
        Assert.Equal(new Value.Guid(id), row["Key"]);
        Assert.Equal(new Value.Bool(true), row["Active"]);
        Assert.Equal(new Value.Text("ada"), row["Login"]);
        Assert.True(row["Created"] is Value.DateTime);
        var stored = converter.ReverseConvert(row);
        Assert.Equal(new Value.Guid(id), stored["Key"]);
        Assert.Equal(new Value.Bool(true), stored["Active"]);
        Assert.Equal(new Value.Text("ada"), stored["Login"]);
        Assert.Equal(new Value.Text("2020-01-02T03:04:05.000Z"), stored["Created"]);
    }
}
