using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteStatementRendererTests
{
    static StatementTextRenderer Renderer { get; } =
        new(t => new SqliteStatementRenderer(t));

    [Fact]
    public void RendersInsertedIdAndCreateTable()
    {
        Assert.Equal(
            "SELECT last_insert_rowid() AS id",
            Renderer.Render(new SelectInsertedIdStatement("users", "id")));

        var create = new CreateTableStatement(
            "orders",
            [
                new Column("id", "INTEGER", new ColumnDefault("0", "df_id"), new NotNull("nn_id")),
                new Column("user_id", "INTEGER")
            ],
            [
                new PrimaryKey("pk_orders", ["id"]),
                new ForeignKey("fk_orders_user", ["user_id"], "users", ["id"], ReferentialAction.Cascade)
            ]);
        Assert.Equal(Renderer.Render(create), new SqliteCreateTableStatementRenderer().Render(create));
        Assert.Equal(
            "SELECT id FROM users",
            Renderer.Render(new SelectStatement(new() { Table = "users", Fields = ["id"] })));
    }

    [Fact]
    public void RendersCanonicalTypes()
    {
        var create = new CreateTableStatement("users", [new Column("id", "int", new ColumnDefault("0"), new NotNull(), true)]);
        Assert.Equal("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL)", Renderer.Render(create));
        Assert.Equal(
            "CREATE TABLE users (id int DEFAULT 0 NOT NULL)",
            DefaultStatementTextRenderer.Instance.Render(create));
        Assert.Equal(
            "CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT)",
            Renderer.Render(new CreateTableStatement("users", [new Column("id", "int", null, null, true)], [new PrimaryKey("pk_users", ["id"])])));
    }
}
