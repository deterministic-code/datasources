using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlStatementRendererTests
{
    static StatementTextRenderer Renderer { get; } =
        new(t => new MysqlStatementRenderer(t));

    [Fact]
    public void RendersInsertedIdAndCreateTable()
    {
        Assert.Equal(
            "SELECT LAST_INSERT_ID() AS id",
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
        Assert.Equal(Renderer.Render(create), new MysqlCreateTableStatementRenderer().Render(create));
        Assert.Equal(
            "SELECT id FROM users",
            Renderer.Render(new SelectStatement(new() { Table = "users", Fields = ["id"] })));
    }

    [Fact]
    public void RendersTypeDefaultAndIdentity()
    {
        var create = new CreateTableStatement("users", [new Column("id", "int", new ColumnDefault("0"), new NotNull(), true)]);
        Assert.Equal("CREATE TABLE users (id INT AUTO_INCREMENT DEFAULT 0 NOT NULL)", Renderer.Render(create));
        Assert.Equal(
            "CREATE TABLE users (id int DEFAULT 0 NOT NULL)",
            DefaultStatementTextRenderer.Instance.Render(create));
    }

    [Fact]
    public void RendersBulkUpdateAsJoin()
    {
        var update = new BulkUpdateStatement(
            "parents",
            ["label"],
            ["id"],
            [
                ([new Parameter("label", new Value.Text("a"))], [new Parameter("id", new Value.Int(1))]),
                ([new Parameter("label", new Value.Text("b"))], [new Parameter("id", new Value.Int(2))])
            ]);
        Assert.Equal(
            "UPDATE parents JOIN (SELECT $label__0 AS label, $id__0 AS id UNION ALL SELECT $label__1 AS label, $id__1 AS id) AS data ON parents.id = data.id SET parents.label = data.label",
            Renderer.Render(update));
    }

    [Fact]
    public void RendersCreateReadUpdateStoredProcedureStatements()
    {
        Assert.Contains("CREATE PROCEDURE CreateUsers", Renderer.Render(
            new CreateCreateStoredProcedureStatement("CreateUsers", "users", ["id", "name"], [new RoutineParameter("name", "TEXT")], "id", "INT", ["name"])));
        Assert.Contains("ORDER BY t.id", Renderer.Render(
            new CreateReadStoredProcedureStatement("ReadUsers", "users", ["id", "name"], [], "id", "INT", [], [])));
        Assert.Contains("WHERE t.id = id", Renderer.Render(
            new CreateReadStoredProcedureStatement("ReadUsersById", "users", ["id", "name"], [new RoutineParameter("id", "INT")], "id", "INT", [], ["id"])));
        Assert.Contains("WHERE t.id = id AND t.version = version", Renderer.Render(
            new CreateUpdateStoredProcedureStatement(
                "UpdateUsersByIdVersion",
                "users",
                ["id", "name", "version"],
                [new RoutineParameter("id", "INT"), new RoutineParameter("version", "INT"), new RoutineParameter("name", "TEXT")],
                "id",
                "INT",
                ["name"],
                ["id", "version"])));
        Assert.Contains("JSON_TABLE", Renderer.Render(
            new CreateBulkDeleteStoredProcedureStatement(
                "BulkDeleteUsers",
                "users",
                [new RoutineParameter("id", "INT")],
                new RoutineParameter("payload", "JSON"),
                "id",
                "INT")));
    }
}
