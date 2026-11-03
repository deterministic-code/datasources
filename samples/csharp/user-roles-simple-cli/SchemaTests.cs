using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace UserRolesSimpleCli.Tests;

public sealed class SchemaTests
{
    [Fact]
    public void CreatesTablesWithForeignKeys()
    {
        var sql = string.Join("\n", SchemaSpec.Tables("user-roles-cli", static id => id).Select(table => DefaultStatementTextRenderer.Instance.Render(table)));
        Assert.Contains("FOREIGN KEY (user_id) REFERENCES", sql);
        Assert.Contains("FOREIGN KEY (group_id) REFERENCES", sql);
        Assert.Contains("FOREIGN KEY (bad_role_id) REFERENCES", sql);
        Assert.Contains("ugly_name", sql);
        Assert.DoesNotContain("password", DefaultStatementTextRenderer.Instance.Render(Program.ListQuery()));
    }

    [Fact]
    public void SqliteUsesIntegerPrimaryKey()
    {
        var users = SchemaSpec.Tables("user-roles-cli", static id => id)[0];
        Assert.True(users.Columns[0].Identity);
        Assert.Contains("CONSTRAINT pk_users PRIMARY KEY", DefaultStatementTextRenderer.Instance.Render(users));
    }

    [Fact]
    public void MysqlUsesIdentityFlag() =>
        Assert.True(SchemaSpec.Tables("user-roles-cli", static id => id)[0].Columns[0].Identity);

    [Fact]
    public void PostgresUsesIdentityFlag() =>
        Assert.True(SchemaSpec.Tables("user-roles-cli", static id => id)[0].Columns[0].Identity);

    [Fact]
    public void MapsColumnTypes()
    {
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" TEXT, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" REAL, \"created\" TEXT, \"version\" BLOB)",
            UserTable("sqlite"));
        Assert.Equal(
            "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
            UserTable("mysql"));
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)",
            UserTable("postgres"));
    }

    static string UserTable(string provider)
    {
        (string Name, string Type, bool Id)[] columns =
        [
            ("id", "int", true),
            ("uuid", "uuid", false),
            ("username", "string", false),
            ("is_enabled", "bool", false),
            ("balance", "decimal", false),
            ("location", "float", false),
            ("created", "datetime", false),
            ("version", "binary", false),
        ];
        var driver = Driver(provider);
        return driver.StatementRenderer!.Render(new CreateTableStatement(
            "user",
            columns.Select(column => new Column(
                column.Name,
                column.Id ? $"{driver.ColumnType(column.Type)} PRIMARY KEY" : driver.ColumnType(column.Type))).ToArray()));
    }

    static IDriver Driver(string provider)
    {
        DatasourceFactory.Register("sqlite", new SqliteDriver());
        DatasourceFactory.Register("mysql", new MysqlDriver());
        DatasourceFactory.Register("postgres", new PostgresDriver());
        return provider switch
        {
            "sqlite" => new SqliteDriver(),
            "mysql" => new MysqlDriver(),
            "postgres" => new PostgresDriver(),
            _ => throw new InvalidOperationException($"unknown provider {provider}")
        };
    }

    [Fact]
    public void MapsUglyColumns()
    {
        Assert.Equal("ugly_name", Program.Names.Map("name"));
        Assert.Equal("bad_role_id", Program.Names.Map("role_id"));
        Assert.Equal("username", Program.Names.Map("username"));
        Assert.Equal("name", Program.Names.ReverseMap("ugly_name"));
        Assert.Equal("role_id", Program.Names.ReverseMap("bad_role_id"));
        Assert.Equal("username", Program.Names.ReverseMap("username"));
    }

    [Fact]
    public void MysqlProceduresWriteMappedColumns()
    {
        using var stream = typeof(Program).Assembly.GetManifestResourceStream("mysql-procedures.sql")!;
        using var reader = new StreamReader(stream);
        var sql = reader.ReadToEnd();
        Assert.Contains("INSERT INTO `groups` (ugly_name", sql);
        Assert.Contains("INSERT INTO `user_roles` (user_id, bad_role_id", sql);
    }

    [Fact]
    public void JoinSelectMatchesContract() =>
        Assert.Equal(
            "SELECT users.id AS id, users.username AS username, users.is_enabled AS is_enabled, users.created AS created, roles.id AS role_id, roles.name AS role_name, roles.is_enabled AS role_is_enabled, groups.id AS group_id, groups.ugly_name AS group_name, groups.is_enabled AS group_is_enabled FROM users LEFT JOIN user_roles ON users.id = user_roles.user_id LEFT JOIN roles ON user_roles.bad_role_id = roles.id LEFT JOIN user_groups ON users.id = user_groups.user_id LEFT JOIN groups ON user_groups.group_id = groups.id ORDER BY users.id ASC, roles.id ASC, groups.id ASC",
            DefaultStatementTextRenderer.Instance.Render(Program.ListQuery()));
}
