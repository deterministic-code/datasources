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
        Assert.Equal(
            "INSERT INTO users (name) VALUES (:name); SELECT LAST_INSERT_ID() AS id",
            Renderer.Render(new InsertStatement("users", [new Parameter("name", "ada")]) { Returning = "id" }));
        Assert.Equal(
            "INSERT INTO users (name) VALUES (:name__0), (:name__1); SELECT LAST_INSERT_ID() AS id UNION ALL SELECT LAST_INSERT_ID() + 1 AS id",
            Renderer.Render(new BulkInsertStatement(
                "users",
                ["name"],
                [[new Parameter("name", "ada")], [new Parameter("name", "bea")]]) { Returning = "id" }));

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
    public void RendersNamedDefaultWithoutConstraint()
    {
        var sql = Renderer.Render(new CreateTableStatement("genders", [
            new Column("uuid", "uuid", new ColumnDefault("(UUID())", "df_uuid"), new NotNull("nn_uuid"))
        ]));
        Assert.Equal("""
            CREATE TABLE genders (
                uuid CHAR(36) DEFAULT (UUID()) NOT NULL
            )
            """, sql);
        Assert.DoesNotContain("CONSTRAINT", sql);
    }

    [Fact]
    public void RendersTypeDefaultAndIdentity()
    {
        var create = new CreateTableStatement("users", [new Column("id", "int", new ColumnDefault("0"), new NotNull(), true)]);
        Assert.Equal("""
            CREATE TABLE users (
                id INT AUTO_INCREMENT DEFAULT 0 NOT NULL
            )
            """, Renderer.Render(create));
        Assert.Equal(
            """
            CREATE TABLE users (
                id int DEFAULT 0 NOT NULL
            )
            """,
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

    [Fact]
    public void RendersAMysqlTableRebuild()
    {
        var rebuilt = Renderer.Render(new RebuildTableStatement(
            "users",
            new CreateTableStatement("users", [new Column("id", "INT"), new Column("age", "INT")]),
            [new RebuildColumn("id", "id"), new RebuildColumn("age", "age", "INT")],
            [new CreateIndexStatement("ix_users_age", "users", ["age"])]));
        Assert.StartsWith("SET FOREIGN_KEY_CHECKS = 0", rebuilt);
        Assert.Contains("users__new", rebuilt);
        Assert.Contains("CAST", rebuilt);
        Assert.Contains("SIGNED", rebuilt);
        Assert.Contains("ix_users_age", rebuilt);
        var indexAt = rebuilt.IndexOf("ix_users_age", StringComparison.Ordinal);
        var renameAt = rebuilt.IndexOf("RENAME TABLE", StringComparison.Ordinal);
        var dropAt = rebuilt.IndexOf("users__old", StringComparison.Ordinal);
        Assert.True(indexAt >= 0 && indexAt < renameAt && renameAt < dropAt);
        Assert.EndsWith("SET FOREIGN_KEY_CHECKS = 1", rebuilt);
    }

    [Fact]
    public void ShortensIndexForeignKeyTriggerAndConstraintNamesTheSameWayOnDrop()
    {
        const string index = "game_journey_settings_game_journey_setting_category_type_id_index";
        const string foreignKey = "game_journey_settings_game_journey_setting_category_type_id_foreign_key";
        const string trigger = "trg_game_journey_setting_category_preference_option_values_updated_at";
        const string unique = "game_journey_settings_external_reference_code_value_unique_constraint";
        Same(Renderer.Render(new CreateIndexStatement(index, "game_journey_settings", ["category_type_id"])), index);
        var uniqueIndex = Renderer.Render(new CreateIndexStatement(index, "game_journey_settings", ["category_type_id"], true));
        Same(uniqueIndex, index);
        Assert.StartsWith("CREATE UNIQUE INDEX ", uniqueIndex);
        Same(Renderer.Render(new DropIndexStatement(index, "game_journey_settings")), index);
        Same(Renderer.Render(new ForeignKeyStatement("orders", foreignKey, ["category_type_id"], "categories", ["id"])), foreignKey);
        Same(Renderer.Render(new ForeignKeyStatement("orders", foreignKey, ["category_type_id"], "categories", ["id"], "drop")), foreignKey);
        Same(Renderer.Render(new CreateTriggerStatement("game_journey_settings", trigger, "updated", "id")), trigger);
        Same(Renderer.Render(new DropTriggerStatement(trigger, "game_journey_settings")), trigger);
        Same(Renderer.Render(new CreateTableStatement("orders", [new Column("category_type_id", "INT")], [
            new ForeignKey(foreignKey, ["category_type_id"], "categories", ["id"]),
            new UniqueConstraint(unique, ["category_type_id"]),
        ])), foreignKey);
        Same(Renderer.Render(new AlterTableStatement("orders", new AlterTableStatement.DropConstraint(unique))), unique);
        Same(Renderer.Render(new AlterTableStatement("orders", new AlterTableStatement.AddConstraint(new UniqueConstraint(unique, ["category_type_id"])))), unique);
        Assert.Equal(new string('a', 64), MysqlIdentifier.Fit(new string('a', 64)));
        Assert.Equal(new string('a', 57) + "_2dd603", MysqlIdentifier.Fit(new string('a', 65)));
        Assert.Equal(
            "game_journey_settings_game_journey_setting_category_d88931_index",
            MysqlIdentifier.Fit(index));
        Assert.Equal(
            "trg_game_journey_setting_category_preference_o_8e15ab_updated_at",
            MysqlIdentifier.Fit(trigger));
        Assert.Equal(
            "notification-types-channel-name-and-something-61b8c2-foreign-key",
            MysqlIdentifier.Fit("notification-types-channel-name-and-something-very-long-foreign-key"));
        Assert.Equal(
            new string('a', 56) + "_a955d9",
            MysqlIdentifier.Fit(new string('a', 56) + "_" + new string('b', 9)));
        Assert.Equal(
            "GameJourneySettingsGameJourneySettingCategoryTy_aa3be2ForeignKey",
            MysqlIdentifier.Fit("GameJourneySettingsGameJourneySettingCategoryTypeIdentifierForeignKey"));
        Assert.Equal(
            "GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_3bfc58_INDEX",
            MysqlIdentifier.Fit("GAME_JOURNEY_SETTINGS_GAME_JOURNEY_SETTING_CATEGORY_TYPE_ID_INDEX"));
    }

    static void Same(string sql, string name)
    {
        var fitted = MysqlIdentifier.Fit(name);
        Assert.True(fitted.Length <= 64);
        Assert.NotEqual(name, fitted);
        Assert.Contains(fitted, sql);
        Assert.DoesNotContain(name, sql);
    }
}
