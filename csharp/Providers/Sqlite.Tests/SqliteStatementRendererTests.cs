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
        Assert.Equal(
            "INSERT INTO users (name) VALUES ($name) RETURNING last_insert_rowid() AS id",
            Renderer.Render(new InsertStatement("users", [new Parameter("name", "ada")]) { Returning = "id" }));
        Assert.Equal(
            "INSERT INTO users (name) VALUES ($name__0) RETURNING last_insert_rowid() AS id",
            Renderer.Render(new BulkInsertStatement("users", ["name"], [[new Parameter("name", "ada")]]) { Returning = "id" }));

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
        Assert.Equal("""
            CREATE TABLE "users" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT DEFAULT 0 NOT NULL
            )
            """, Renderer.Render(create));
        Assert.Equal(
            """
            CREATE TABLE users (
                id int DEFAULT 0 NOT NULL
            )
            """,
            DefaultStatementTextRenderer.Instance.Render(create));
        Assert.Equal(
            """
            CREATE TABLE "users" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT
            )
            """,
            Renderer.Render(new CreateTableStatement("users", [new Column("id", "int", null, null, true)], [new PrimaryKey("pk_users", ["id"])])));
    }

    [Fact]
    public void RendersAlterAddColumnAndThrowsOnAlterColumn()
    {
        Assert.Equal(
            "ALTER TABLE \"users\" ADD COLUMN \"email\" TEXT NOT NULL",
            Renderer.Render(new AlterTableStatement("users", new AlterTableStatement.AddColumn(new Column("email", "TEXT", NotNull: new NotNull())))));
        Assert.Throws<InvalidOperationException>(() =>
            Renderer.Render(new AlterTableStatement("users", new AlterTableStatement.AlterColumn(new Column("age", "INTEGER")))));
        Assert.Equal(
            "CREATE UNIQUE INDEX \"ux_email\" ON \"users\" (\"email\")",
            Renderer.Render(new AlterTableStatement("users", new AlterTableStatement.AddConstraint(new UniqueConstraint("ux_email", ["email"])))));
        Assert.Equal(
            "DROP INDEX IF EXISTS \"ux_email\"",
            Renderer.Render(new AlterTableStatement("users", new AlterTableStatement.DropConstraint("ux_email"))));
        Assert.Equal("", Renderer.Render(new ForeignKeyStatement("users", "fk", ["id"], "other", ["id"], "drop")));
        Assert.Throws<InvalidOperationException>(() =>
            Renderer.Render(new ForeignKeyStatement("users", "fk", ["id"], "other", ["id"])));
        var rebuilt = Renderer.Render(new ForeignKeyStatement(
            "users",
            "fk",
            ["id"],
            "other",
            ["id"],
            Create: new CreateTableStatement("users", [new Column("id", "INTEGER")])));
        Assert.Contains("CREATE TABLE \"users__new\"", rebuilt);
        Assert.Contains("INSERT INTO \"users__new\" (\"id\") SELECT \"id\" FROM \"users\"", rebuilt);
        Assert.Contains("DROP TABLE IF EXISTS \"users\"", rebuilt);
        Assert.Contains("ALTER TABLE \"users__new\" RENAME TO \"users\"", rebuilt);
    }

    [Fact]
    public void RendersASqliteTableRebuild()
    {
        var rebuilt = Renderer.Render(new RebuildTableStatement(
            "users",
            new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("age", "INTEGER")]),
            [new RebuildColumn("id", "id"), new RebuildColumn("age", "age", "INTEGER")],
            [new CreateIndexStatement("ix_users_age", "users", ["age"])]));
        Assert.Contains("CREATE TABLE \"users__new\"", rebuilt);
        Assert.Contains("INSERT INTO \"users__new\" (\"id\", \"age\") SELECT \"id\", CAST(\"age\" AS INTEGER) FROM \"users\"", rebuilt);
        var createAt = rebuilt.IndexOf("CREATE TABLE \"users__new\"", StringComparison.Ordinal);
        var dropAt = rebuilt.IndexOf("DROP TABLE IF EXISTS \"users\"", StringComparison.Ordinal);
        var renameAt = rebuilt.IndexOf("ALTER TABLE \"users__new\" RENAME TO \"users\"", StringComparison.Ordinal);
        var indexAt = rebuilt.IndexOf("CREATE INDEX \"ix_users_age\" ON \"users\" (\"age\")", StringComparison.Ordinal);
        Assert.True(createAt >= 0 && createAt < dropAt && dropAt < renameAt && renameAt < indexAt);
        var copied = Renderer.Render(new RebuildTableStatement(
            "users",
            new CreateTableStatement("users", [new Column("id", "INTEGER"), new Column("note", "TEXT"), new Column("flag", "INTEGER")]),
            [new RebuildColumn("id", "id"), new RebuildColumn("note"), new RebuildColumn("flag", null, null, "0")],
            [new CreateIndexStatement("ix_users_note", "users", ["note"]), new CreateTriggerStatement("users", "trg_users", "note", "id")]));
        Assert.Contains("SELECT \"id\", NULL, 0 FROM \"users\"", copied);
        Assert.Contains("trg_users", copied);
        Assert.Throws<InvalidOperationException>(() => Renderer.Render(new RebuildTableStatement(
            "users",
            new CreateTableStatement("users", [new Column("id", "INTEGER")]),
            [new RebuildColumn("id", "id")],
            [new DropTableStatement("users")])));
    }

    [Fact]
    public void RendersStoredProcedureStatementsEmpty()
    {
        Assert.Equal(
            "",
            Renderer.Render(new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id"], [], "id", "INT", [], [])));
    }
}
