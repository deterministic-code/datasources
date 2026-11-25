using Backend.Types.Datasource;
using Backend.Types.View;
using Datasources;
using Datasources.Sqlite;

public sealed class BadgeViewTests
{
    [Fact]
    public async Task CreatesReadsUpdatesAndDeletes()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await ds.CreateConnection();
        await connection.Open();
        await Schema(connection);
        var repo = new BadgeViewsRepository(connection);
        var created = await repo.Create(new BadgeView
        {
            BadgeId = Guid.Parse("11111111-1111-1111-1111-111111111111"),
            Code = "alpha",
            Name = "north",
            Aliases = [new BadgeAlias { Name = "uptown" }],
            Settings = new Dictionary<string, string> { ["color"] = "blue" },
            Nodes = [new BadgeNode { Name = "dock" }],
        });
        var loaded = await repo.ReadSingleById(new Row { ["BadgeId"] = created.BadgeId, ["Code"] = created.Code });
        Assert.NotNull(loaded);
        Assert.Equal("north", loaded.Name);
        Assert.Equal("uptown", Assert.Single(loaded.Aliases).Name);
        Assert.Equal("blue", loaded.Settings["color"]);
        Assert.Equal("dock", Assert.Single(loaded.Nodes).Name);
        loaded.Name = "south";
        loaded.Aliases[0].Name = "downtown";
        loaded.Settings["color"] = "red";
        loaded.Nodes[0].Name = "pier";
        await repo.Update(loaded);
        var updated = await repo.ReadSingleById(new Row { ["BadgeId"] = loaded.BadgeId, ["Code"] = loaded.Code });
        Assert.NotNull(updated);
        Assert.Equal("south", updated.Name);
        Assert.Equal("downtown", Assert.Single(updated.Aliases).Name);
        Assert.Equal("red", updated.Settings["color"]);
        Assert.Equal("pier", Assert.Single(updated.Nodes).Name);
        await repo.Delete(updated);
        Assert.Null(await repo.ReadSingleById(new Row { ["BadgeId"] = updated.BadgeId, ["Code"] = updated.Code }));
    }

    static async Task Schema(IConnection connection)
    {
        foreach (var sql in new[]
        {
            """
            CREATE TABLE "badges" (
                "badge_id" TEXT NOT NULL,
                "code" VARCHAR(64) NOT NULL,
                "name" VARCHAR(128) NOT NULL,
                CONSTRAINT "badges_primary_key" PRIMARY KEY ("badge_id", "code")
            )
            """,
            """
            CREATE TABLE "badge_aliases" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_badge_id" TEXT NOT NULL,
                "parent_code" TEXT NOT NULL,
                "name" VARCHAR(128) NOT NULL
            )
            """,
            """
            CREATE TABLE "badge_settings" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_badge_id" TEXT NOT NULL,
                "parent_code" TEXT NOT NULL,
                "name" VARCHAR(64) NOT NULL,
                "value" VARCHAR(256) NOT NULL
            )
            """,
            """
            CREATE TABLE "badge_nodes" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_badge_id" TEXT NOT NULL,
                "parent_code" TEXT NOT NULL,
                "name" VARCHAR(128) NOT NULL
            )
            """,
        })
            await connection.CreateCommand().Execute(Native.N(sql));
    }
}
