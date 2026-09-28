using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteStatementFactoryTests
{
    [Fact]
    public void TransformsCreateTableAndPassesOtherStatementsThrough()
    {
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
        var factory = new SqliteStatementFactory();
        var transformed = Assert.IsType<CreateTableStatement>(factory.Transform(create));
        Assert.Null(transformed.Table);
        Assert.Equal(create.GetText(), transformed.GetText());
        Assert.Same(transformed, factory.Transform(transformed));

        var raw = new CreateTableStatement("x");
        Assert.Same(raw, factory.Transform(raw));
        var select = new SelectStatement("SELECT id FROM users");
        Assert.Same(select, factory.Transform(select));
        Assert.Equal("SELECT last_insert_rowid() AS id", factory.Transform(new InsertedIdStatement("users", "id")).GetText());
    }

    [Fact]
    public void RendersCanonicalTypes()
    {
        var create = new CreateTableStatement("users", [new Column("id", "int", new ColumnDefault("0"), new NotNull(), true)]);
        var text = new SqliteStatementFactory().Transform(create).GetText();
        Assert.Equal("CREATE TABLE users (id INTEGER DEFAULT 0 NOT NULL)", text);
        Assert.Equal("CREATE TABLE users (id int DEFAULT 0 NOT NULL)", create.GetText());
    }
}
