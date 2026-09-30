using Datasources;
using Datasources.Mysql;
using Datasources.SqlSpecs;
using MySqlConnector;

namespace Datasources.Mysql.Tests;

public class MysqlCommandIntegrationTests
{
    [Fact]
    public async Task ReturnAndOutputParameters()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var isolated = await Isolate(url);
        try
        {
            var conn = new MysqlConnection(new MysqlConnectionOptions
            {
                Url = isolated.Url,
            });
            await conn.Open();
            var command = new MysqlCommand(conn);
            await FillOutputParameter(command);
            await command.Execute(Native.N("""
                CREATE FUNCTION add_one(n INT) RETURNS INT DETERMINISTIC
                RETURN n + 1
                """));
            var n = new Parameter("n", 4);
            var ret = await command.Execute(new ReturnStatement("add_one", [n]), [n]);
            Assert.Equal(new Value.Int(5), ret.Result);
        }
        finally
        {
            await isolated.Cleanup();
        }
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var url = Environment.GetEnvironmentVariable("MYSQL_URL");
        if (string.IsNullOrEmpty(url)) return;
        var dialect = SqlSpec.Dialect.Load("mysql", static statement => new MysqlStatementFactory().Transform(statement).GetText());
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
        {
            var isolated = await Isolate(url);
            try
            {
                await SqlSpec.Run(path, dialect, _ =>
                {
                    var conn = new MysqlConnection(new MysqlConnectionOptions
                    {
                        Url = isolated.Url,
                    });
                    return new SqlSpec.Session(conn, new MysqlCommand(conn));
                }, () => new MysqlCommand(new MysqlConnection()));
            }
            finally
            {
                await isolated.Cleanup();
            }
        }
    }

    static async Task FillOutputParameter(ICommand command)
    {
        await command.Execute(Native.N("""
            CREATE PROCEDURE bump(IN start_n INT, OUT result INT)
            BEGIN
              SET result = start_n + 1;
            END
            """));
        var start = new Parameter("start_n", 4);
        var result = new Parameter("result", ParameterDirection.Output);
        var call = await command.Execute(new ExecuteStoredProcedureStatement("bump", [start, result]), [start, result]);
        Assert.Equal(new Value.Int(5), call.Parameters![1].Value);
    }

    static async Task<(string Url, Func<Task> Cleanup)> Isolate(string baseUrl)
    {
        var name = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE DATABASE `{name}`";
            await create.ExecuteNonQueryAsync();
        }
        var isolated = ReplaceDatabase(baseUrl, name);
        return (isolated, async () =>
        {
            await using var drop = new MySqlConnection(MysqlConnectionString.ToConnectionString(baseUrl));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP DATABASE `{name}`";
            await cmd.ExecuteNonQueryAsync();
        });
    }

    static string ReplaceDatabase(string url, string name)
    {
        if (!url.StartsWith("mysql://", StringComparison.OrdinalIgnoreCase))
            return $"{url};Database={name}";
        var uri = new Uri(url);
        var builder = new UriBuilder(uri) { Path = name };
        return builder.Uri.ToString();
    }
}
