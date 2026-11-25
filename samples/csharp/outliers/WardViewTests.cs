using Backend.Types.Datasource;
using Backend.Types.View;
using Datasources;
using Datasources.Sqlite;

public sealed class WardViewTests
{
    [Fact]
    public async Task CreatesReadsUpdatesAndDeletes()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await ds.CreateConnection();
        await connection.Open();
        await Schema(connection);
        var repo = new WardViewsRepository(connection);
        var created = await repo.Create(new WardView
        {
            RegionId = 1,
            WardId = 1,
            Name = "north",
            Aliases = [new WardAlias { Name = "uptown" }],
            Settings = new Dictionary<string, string> { ["color"] = "blue" },
            Nodes = [new WardNode { Name = "dock" }],
        });
        var loaded = await repo.ReadSingleById(new Row { ["RegionId"] = created.RegionId, ["WardId"] = created.WardId });
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
        var updated = await repo.ReadSingleById(new Row { ["RegionId"] = loaded.RegionId, ["WardId"] = loaded.WardId });
        Assert.NotNull(updated);
        Assert.Equal("south", updated.Name);
        Assert.Equal("downtown", Assert.Single(updated.Aliases).Name);
        Assert.Equal("red", updated.Settings["color"]);
        Assert.Equal("pier", Assert.Single(updated.Nodes).Name);
        await repo.Delete(updated);
        Assert.Null(await repo.ReadSingleById(new Row { ["RegionId"] = updated.RegionId, ["WardId"] = updated.WardId }));
    }

    static async Task Schema(IConnection connection)
    {
        foreach (var sql in new[]
        {
            """
            CREATE TABLE "wards" (
                "region_id" INTEGER NOT NULL,
                "ward_id" INTEGER NOT NULL,
                "name" VARCHAR(128) NOT NULL,
                CONSTRAINT "wards_primary_key" PRIMARY KEY ("region_id", "ward_id")
            )
            """,
            """
            CREATE TABLE "ward_aliases" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_region_id" INTEGER NOT NULL,
                "parent_ward_id" INTEGER NOT NULL,
                "name" VARCHAR(128) NOT NULL
            )
            """,
            """
            CREATE TABLE "ward_settings" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_region_id" INTEGER NOT NULL,
                "parent_ward_id" INTEGER NOT NULL,
                "name" VARCHAR(64) NOT NULL,
                "value" VARCHAR(256) NOT NULL
            )
            """,
            """
            CREATE TABLE "ward_nodes" (
                "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                "parent_region_id" INTEGER NOT NULL,
                "parent_ward_id" INTEGER NOT NULL,
                "name" VARCHAR(128) NOT NULL
            )
            """,
        })
            await connection.CreateCommand().Execute(Native.N(sql));
    }
}
