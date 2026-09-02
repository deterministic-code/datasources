using Datasources.Postgres;
using Datasources.SqlSpecs;
using Npgsql;

namespace Datasources.Postgres.Tests;

public class PostgresRepositorySpecTests
{
    [Fact]
    public async Task RepositorySpecs()
    {
        var url = Environment.GetEnvironmentVariable("POSTGRES_URL");
        if (string.IsNullOrEmpty(url)) return;
        _ = PostgresTypes.All;
        foreach (var path in RepositorySpec.Files())
        {
            var schema = $"ds_{Guid.NewGuid():N}"[..18];
            await using (var admin = new NpgsqlConnection(url))
            {
                await admin.OpenAsync();
                await using var create = admin.CreateCommand();
                create.CommandText = $"CREATE SCHEMA {schema}";
                await create.ExecuteNonQueryAsync();
            }
            try
            {
                await RepositorySpec.Run(path, () =>
                {
                    var conn = new PostgresConnection(new PostgresConnectionOptions
                    {
                        Url = url,
                        Schema = schema,
                        Driver = new PostgresDriver(schema),
                    });
                    return new RepositorySpec.Session(conn, new PostgresCommand(conn));
                }, (table, columns) => PostgresStatementBuilder.CreateTable(table,
                    [.. columns.Select(column => new PostgresColumn(column.Name, column.Type, column.Id))]).GetText(),
                    PostgresStatementBuilder.Quote);
            }
            finally
            {
                await using var drop = new NpgsqlConnection(url);
                await drop.OpenAsync();
                await using var cmd = drop.CreateCommand();
                cmd.CommandText = $"DROP SCHEMA {schema} CASCADE";
                await cmd.ExecuteNonQueryAsync();
            }
        }
    }
}
