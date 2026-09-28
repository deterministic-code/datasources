using Datasources;
using Datasources.Postgres;
using Datasources.SqlSpecs;
using Npgsql;

namespace Datasources.Postgres.Tests;

public class PostgresCommandIntegrationTests
{
    [Fact]
    public async Task ReturnAndOutputParameters()
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
            });
            await conn.Open();
            var command = new PostgresCommand(conn);
            await command.Execute("""
                CREATE PROCEDURE bump(INOUT result INT)
                LANGUAGE plpgsql
                AS $$
                BEGIN
                  result := result + 1;
                END;
                $$
                """);
            await command.Execute("""
                CREATE FUNCTION add_one(n INT) RETURNS INT
                LANGUAGE sql
                AS $$ SELECT n + 1 $$
                """);
            var result = new Parameter("result", 4, ParameterDirection.InputOutput);
            var call = await command.Execute(new ExecuteStoredProcedureStatement("bump", [result]).GetText(), [result]);
            Assert.Equal(new Value.Int(5), call.Parameters![0].Value);
            var n = new Parameter("n", 4);
            var ret = await command.Execute(new ReturnStatement("add_one", [n]).GetText(), [n]);
            Assert.Equal(new Value.Int(5), ret.Result);
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
            });
            await conn.Open();
            var command = new PostgresCommand(conn);
            foreach (var statement in SchemaSpec.Tables("postgres-items", static id => $"\"{id.Replace("\"", "\"\"")}\""))
                await command.Execute(statement);
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
        var dialect = SqlSpec.Dialect.Load("postgres", static statement => new PostgresStatementFactory().Transform(statement).GetText());
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml").Where(path => Path.GetFileName(path) != "schemas.yaml"))
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
                    });
                    return new SqlSpec.Session(conn, new PostgresCommand(conn));
                }, () => new PostgresCommand(new PostgresConnection()));
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
