using Datasources.Postgres;
using Datasources.SqlSpecs;
using Npgsql;

namespace Datasources.Postgres.Tests;

public class PostgresCommandIntegrationTests
{
    [Fact]
    public async Task SharedSpecs()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        var dialect = SqlSpec.Dialect.Load("postgres");
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml"))
        {
            var schema = $"ds_{Guid.NewGuid():N}"[..18];
            await using (var admin = new NpgsqlConnection(PostgresDriver.ToConnectionString(url)))
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
                await using var drop = new NpgsqlConnection(PostgresDriver.ToConnectionString(url));
                await drop.OpenAsync();
                await using var cmd = drop.CreateCommand();
                cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
                await cmd.ExecuteNonQueryAsync();
            }
        }
    }
}
