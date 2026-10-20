using Datasources;

namespace UserRolesSimpleCli.Tests;

public sealed class AppTests
{
    static readonly DateTime Frozen = new(2026, 9, 22, 0, 0, 0, DateTimeKind.Utc);
    static readonly byte[] Token = [1, 2, 3];

    [Fact]
    public async Task WritesSqliteInsertsAndLists()
    {
        var (app, command) = Create("sqlite");
        command.Reader = new MockReader<Row>([new Row { ["id"] = 7 }]);
        await app.Run(new AddUser("alice", "secret"));
        await app.Run(new AddRole("admin"));
        await app.Run(new AddGroup("staff"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }, new Row { ["id"] = 2 }]);
        await app.Run(new AddUserRole("alice", "admin"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }, new Row { ["id"] = 3 }]);
        await app.Run(new AddUserGroup("alice", "staff"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }]);
        await app.Run(new DisableUser("alice"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("INSERT INTO", StringComparison.Ordinal) && call.Text.Contains("users", StringComparison.Ordinal));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("ugly_name"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("bad_role_id"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("UPDATE", StringComparison.Ordinal) && call.Text.Contains("users", StringComparison.Ordinal));
        command.Reader = new MockReader<Row>(
        [
            null,
            new Row
            {
                ["id"] = 1,
                ["username"] = "alice",
                ["is_enabled"] = false,
                ["created"] = "2026-09-22T00:00:00.000Z",
                ["role_id"] = 2,
                ["role_name"] = "admin",
                ["role_is_enabled"] = true,
                ["group_id"] = 3,
                ["group_name"] = "staff",
                ["group_is_enabled"] = true
            }
        ]);
        var json = await app.Run(new ListUsersRolesGroups());
        Assert.Contains("\"username\":\"alice\"", json);
        Assert.Contains("\"name\":\"admin\"", json);
        Assert.Contains("\"name\":\"staff\"", json);
        Assert.DoesNotContain("password", json);
        Assert.DoesNotContain("ugly_name", json);
    }

    [Fact]
    public async Task WritesMysqlProcedures()
    {
        var (app, command) = Create("mysql");
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }]);
        await app.Run(new AddUser("alice", "secret"));
        await app.Run(new AddRole("admin"));
        await app.Run(new AddGroup("staff"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }, new Row { ["id"] = 2 }]);
        await app.Run(new AddUserRole("alice", "admin"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }, new Row { ["id"] = 3 }]);
        await app.Run(new AddUserGroup("alice", "staff"));
        await app.Run(new EnableUser("alice"));
        await app.Run(new DisableUser("alice"));
        Assert.All(command.ExecuteCalls, call => Assert.StartsWith("CALL ", call.Text));
        Assert.Contains(command.ExecuteCalls, call => call.Text.StartsWith("CALL add_user(", StringComparison.Ordinal));
        Assert.Contains(command.ExecuteCalls, call => call.Text.StartsWith("CALL set_user_enabled(", StringComparison.Ordinal));
    }

    [Fact]
    public async Task MysqlSqlPathInsertsAndUpdates()
    {
        var (app, command) = Create("mysql", false);
        await app.Run(new AddUser("alice", "secret"));
        command.Reader = new MockReader<Row>([new Row { ["id"] = 1 }]);
        await app.Run(new EnableUser("alice"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("INSERT INTO", StringComparison.Ordinal));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("UPDATE", StringComparison.Ordinal));
        Assert.DoesNotContain(command.ExecuteCalls, call => call.Text.StartsWith("CALL ", StringComparison.Ordinal));
    }

    [Fact]
    public async Task MigratesSqliteAndMysql()
    {
        var sqlite = new MockConnection { Provider = "sqlite" };
        var sqliteCmd = new MockCommand(sqlite);
        sqlite.Command = sqliteCmd;
        await Migrate.Apply(sqliteCmd);
        Assert.All(sqliteCmd.ExecuteCalls, call => Assert.StartsWith("CREATE TABLE", call.Text));
        var mysql = new MockConnection { Provider = "mysql" };
        var mysqlCmd = new MockCommand(mysql);
        mysql.Command = mysqlCmd;
        await Migrate.Apply(mysqlCmd);
        Assert.Contains(mysqlCmd.ExecuteCalls, call => call.Text.Contains("CREATE PROCEDURE add_user"));
    }

    [Fact]
    public async Task UsesDefaultClockAndHash()
    {
        var connection = new MockConnection { Provider = "sqlite" };
        var command = new MockCommand(connection);
        connection.Command = command;
        await new App(command).Run(new AddUser("alice", "secret"));
        Assert.Contains(command.ExecuteCalls, call => call.Text.Contains("INSERT INTO", StringComparison.Ordinal));
    }

    [Fact]
    public async Task MissingLookupThrows()
    {
        var (app, command) = Create("sqlite");
        command.Reader = new MockReader<Row>([]);
        await Assert.ThrowsAsync<InvalidOperationException>(() => app.Run(new EnableUser("missing")));
        command.Reader = new MockReader<Row>([new Row { ["id"] = "x" }]);
        await Assert.ThrowsAsync<InvalidOperationException>(() => app.Run(new EnableUser("bad")));
        await Assert.ThrowsAsync<InvalidOperationException>(() => app.Run(new FakeCommand()));
    }

    static (App App, MockCommand Command) Create(string provider, bool procedures = true)
    {
        var connection = new MockConnection { Provider = provider };
        var command = new MockCommand(connection);
        connection.Command = command;
        return (new App(command, () => Frozen, password => $"#{password}", () => Token, procedures), command);
    }

    sealed record FakeCommand : CliCommand
    {
        public override Task<string> Run(App app) => throw new InvalidOperationException("empty command");
    }
}
