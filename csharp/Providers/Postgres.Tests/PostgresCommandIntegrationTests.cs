using Datasources;
using Datasources.Postgres;
using Datasources.SqlSpecs;
using Npgsql;

namespace Datasources.Postgres.Tests;

public class PostgresCommandIntegrationTests
{
    [Fact]
    public async Task NamedParametersBindOnInsert()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var schema = $"ds_{Guid.NewGuid():N}"[..18];
        await using (var admin = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url)))
        {
            await admin.OpenAsync();
            await using var create = admin.CreateCommand();
            create.CommandText = $"CREATE SCHEMA {schema}";
            await create.ExecuteNonQueryAsync();
        }
        try
        {
            var conn = new PostgresConnection(new PostgresConnectionOptions
            {
                Url = url,
                Schema = schema,
                Driver = new PostgresDriver(schema),
            });
            await conn.Open();
            var command = new PostgresCommand(conn);
            await command.Execute("CREATE TABLE items (id int, name text)");
            await command.Execute(
                "INSERT INTO items (id, name) VALUES ($id, $name)",
                [new Parameter("id", 7), new Parameter("name", "bound")]);
            var reader = await command.ExecuteReader(
                "SELECT id, name FROM items WHERE id = $id",
                [new Parameter("id", 7)]);
            Assert.True(await reader.Next());
            Assert.Equal(new Value.Int(7), reader.Current!["id"]);
            Assert.Equal(new Value.Text("bound"), reader.Current["name"]);
        }
        finally
        {
            await using var drop = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url));
            await drop.OpenAsync();
            await using var cmd = drop.CreateCommand();
            cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
            await cmd.ExecuteNonQueryAsync();
        }
    }

    [Fact]
    public async Task SharedSpecs()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var dialect = SqlSpec.Dialect.Load("postgres");
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml"))
        {
            var schema = $"ds_{Guid.NewGuid():N}"[..18];
            await using (var admin = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url)))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE SCHEMA {schema}";
                await create.ExecuteNonQueryAsync();
            }
            try
            {
                await SqlSpec.Run(path, dialect, _ =>
                {
                    var conn = new PostgresConnection(new PostgresConnectionOptions
                    {
                        Url = url,
                        Schema = schema,
                        Driver = new PostgresDriver(schema),
                    });
                    return new SqlSpec.Session(conn, new PostgresCommand(conn));
                }, () => new PostgresCommand(new PostgresConnection(new PostgresConnectionOptions { Driver = new PostgresDriver() })));
            }
            finally
            {
                await using var drop = new NpgsqlConnection(PostgresConnectionString.ToConnectionString(url));
                await drop.OpenAsync();
                await using var cmd = drop.CreateCommand();
                cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
                await cmd.ExecuteNonQueryAsync();
            }
        }
    }
}
