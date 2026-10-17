using Datasources;

namespace UserRolesSimpleCli.Tests;

public sealed class AsUserViewTests
{
    [Fact]
    public void MapsNestedViewsAndFallbackCells()
    {
        var mapped = new FlatToNestedMapper(UsersView.TypeMap).Map(
        [
            Row(
                ("id", 1),
                ("username", "alice"),
                ("is_enabled", true),
                ("created", "2026-01-01T00:00:00.000Z"),
                ("role_id", 2),
                ("role_name", "admin"),
                ("role_is_enabled", 1L),
                ("group_id", 3),
                ("group_name", "staff"),
                ("group_is_enabled", false)
            ),
            Row(
                ("id", 1),
                ("username", "alice"),
                ("is_enabled", true),
                ("created", "2026-01-01T00:00:00.000Z"),
                ("role_id", 4),
                ("role_name", "ops"),
                ("role_is_enabled", 0L),
                ("group_id", Value.Null.Instance),
                ("group_name", Value.Null.Instance),
                ("group_is_enabled", Value.Null.Instance)
            )
        ]);
        var users = AsUserView.FromMapped(mapped);
        Assert.Single(users);
        Assert.Equal("alice", users[0].username);
        Assert.Equal(["admin", "ops"], users[0].roles.Select(role => role.name));
        Assert.Equal(["staff"], users[0].groups.Select(group => group.name));
        Assert.True(users[0].roles[0].is_enabled);
        Assert.False(users[0].roles[1].is_enabled);
    }

    [Fact]
    public void MapsFallbackValueShapes()
    {
        var created = new DateTime(2026, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        var users = AsUserView.FromMapped(
        [
            new Dictionary<string, MappedValue>
            {
                ["id"] = new MappedValue.Cell(new Value.Text("9")),
                ["username"] = new MappedValue.Cell(new Value.Int(1)),
                ["is_enabled"] = new MappedValue.Cell(Value.Null.Instance),
                ["created"] = new MappedValue.Cell(created),
                ["roles"] = new MappedValue.Cell(Value.Null.Instance),
                ["missing"] = new MappedValue.Cell(new Value.Float(1))
            }
        ]);
        Assert.Equal(9, users[0].id);
        Assert.Equal("1", users[0].username);
        Assert.False(users[0].is_enabled);
        Assert.Equal(created.ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'"), users[0].created);
        Assert.Empty(users[0].roles);
        Assert.Empty(users[0].groups);
        Assert.Equal(0, AsUserView.FromMapped(
        [
            new Dictionary<string, MappedValue>
            {
                ["id"] = new MappedValue.Cell(new Value.Float(1.2)),
                ["username"] = new MappedValue.Cell(Value.Null.Instance),
                ["is_enabled"] = new MappedValue.Cell(new Value.Text("x")),
                ["created"] = new MappedValue.Cell(new Value.Text("2026-01-02 03:04:05"))
            }
        ])[0].id);
        Assert.Equal(
            "2026-01-02T03:04:05.000Z",
            AsUserView.FromMapped(
            [
                new Dictionary<string, MappedValue>
                {
                    ["id"] = new MappedValue.Cell(1),
                    ["username"] = new MappedValue.Cell("ada"),
                    ["is_enabled"] = new MappedValue.Cell(true),
                    ["created"] = new MappedValue.Cell(new Value.Text("2026-01-02 03:04:05"))
                }
            ])[0].created);
        var empty = AsUserView.FromMapped([new Dictionary<string, MappedValue>
        {
            ["username"] = new MappedValue.Many([])
        }])[0];
        Assert.Equal(0, empty.id);
        Assert.Equal("", empty.username);
        Assert.False(empty.is_enabled);
        Assert.Equal("", empty.created);
    }

    static Row Row(params (string Key, Value Value)[] cells) =>
        new(cells.Select(cell => new KeyValuePair<string, Value>(cell.Key, cell.Value)));
}
