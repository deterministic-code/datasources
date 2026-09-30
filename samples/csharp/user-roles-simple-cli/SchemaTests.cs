using Datasources;
using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

namespace UserRolesSimpleCli.Tests;

public sealed class SchemaTests
{
    [Theory]
    [InlineData("sqlite")]
    [InlineData("mysql")]
    [InlineData("postgres")]
    public void CreatesTablesWithForeignKeys(string provider)
    {
        var sql = string.Join("\n", Schema.Tables(provider).Select(table => table.GetText()));
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "user_id")}) REFERENCES", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "group_id")}) REFERENCES", sql);
        Assert.Contains($"FOREIGN KEY ({Schema.Ident(provider, "bad_role_id")}) REFERENCES", sql);
        Assert.Contains("ugly_name", sql);
        Assert.DoesNotContain("password", UsersRolesGroupsQuery.Create(provider).GetText());
    }

    [Fact]
    public void SqliteUsesIntegerPrimaryKey()
    {
        var users = Schema.Tables("sqlite")[0];
        Assert.True(users.Columns[0].Identity);
        Assert.Contains("CONSTRAINT pk_users PRIMARY KEY", users.GetText());
    }

    [Fact]
    public void MysqlUsesIdentityFlag() =>
        Assert.True(Schema.Tables("mysql")[0].Columns[0].Identity);

    [Fact]
    public void PostgresUsesIdentityFlag() =>
        Assert.True(Schema.Tables("postgres")[0].Columns[0].Identity);

    [Fact]
    public void QuotesIdentifiersAndMapsColumnTypes()
    {
        Assert.Equal("\"user\"", Schema.Ident("sqlite", "user"));
        Assert.Equal("\"a\"\"b\"", Schema.Ident("postgres", "a\"b"));
        Assert.Equal("`a``b`", Schema.Ident("mysql", "a`b"));
        Assert.Equal("\"users\".\"id\"", Schema.Qualified("sqlite", "users.id"));
        Assert.Equal("`users`.`id`", Schema.Qualified("mysql", "users.id"));

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
        return new CreateTableStatement(
            Schema.Ident(provider, "user"),
            columns.Select(column => new Column(
                Schema.Ident(provider, column.Name),
                column.Id ? $"{driver.ColumnType(column.Type)} PRIMARY KEY" : driver.ColumnType(column.Type))).ToArray()).GetText();
    }

    static IDriver Driver(string provider) => provider switch
    {
        "sqlite" => new SqliteDriver(),
        "mysql" => new MysqlDriver(),
        "postgres" => new PostgresDriver(),
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };

    [Fact]
    public void MapsUglyColumns()
    {
        Assert.Equal("ugly_name", Columns.Names.Map("name"));
        Assert.Equal("bad_role_id", Columns.Names.Map("role_id"));
        Assert.Equal("username", Columns.Names.Map("username"));
        Assert.Equal("name", Columns.Names.ReverseMap("ugly_name"));
        Assert.Equal("role_id", Columns.Names.ReverseMap("bad_role_id"));
        Assert.Equal("username", Columns.Names.ReverseMap("username"));
    }

    [Fact]
    public void PrefixesCreateTableIfNotExists()
    {
        Assert.Equal("CREATE TABLE IF NOT EXISTS users (id INT)", Migrate.IfNotExists("CREATE TABLE users (id INT)"));
        Assert.Equal("SELECT 1", Migrate.IfNotExists("SELECT 1"));
    }

    [Fact]
    public void MysqlProceduresWriteMappedColumns()
    {
        var sql = string.Join("\n", Schema.MysqlProcedures());
        Assert.Contains("INSERT INTO `groups` (ugly_name", sql);
        Assert.Contains("INSERT INTO `user_roles` (user_id, bad_role_id", sql);
    }

    [Fact]
    public void JoinSelectMatchesContract() =>
        Assert.Equal(
            "SELECT \"users\".id AS id, \"users\".username AS username, \"users\".is_enabled AS is_enabled, \"users\".created AS created, \"roles\".id AS role_id, \"roles\".name AS role_name, \"roles\".is_enabled AS role_is_enabled, \"groups\".id AS group_id, \"groups\".ugly_name AS group_name, \"groups\".is_enabled AS group_is_enabled FROM \"users\" LEFT JOIN \"user_roles\" ON \"users\".id = \"user_roles\".user_id LEFT JOIN \"roles\" ON \"user_roles\".bad_role_id = \"roles\".id LEFT JOIN \"user_groups\" ON \"users\".id = \"user_groups\".user_id LEFT JOIN \"groups\" ON \"user_groups\".group_id = \"groups\".id ORDER BY \"users\".id ASC, \"roles\".id ASC, \"groups\".id ASC",
            UsersRolesGroupsQuery.Create("sqlite").GetText());
}
