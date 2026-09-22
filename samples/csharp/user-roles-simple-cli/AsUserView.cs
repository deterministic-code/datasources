using Datasources;

namespace UserRolesSimpleCli;

public static class AsUserView
{
    public static IReadOnlyList<UserView> FromMapped(IEnumerable<Dictionary<string, MappedValue>> rows)
    {
        var users = new List<UserView>();
        foreach (var row in rows)
            users.Add(new UserView(
                Id(row, "id"),
                Text(row, "username"),
                Flag(row, "is_enabled"),
                Stamp(row, "created"),
                Roles(Many(row, "roles")),
                Groups(Many(row, "groups"))));
        users.Sort((left, right) => left.id.CompareTo(right.id));
        return users;
    }

    static RoleView[] Roles(List<Dictionary<string, MappedValue>> rows)
    {
        var views = new List<RoleView>();
        foreach (var row in rows)
            views.Add(new RoleView(Id(row, "id"), Text(row, "name"), Flag(row, "is_enabled")));
        views.Sort((left, right) => left.id.CompareTo(right.id));
        return views.ToArray();
    }

    static GroupView[] Groups(List<Dictionary<string, MappedValue>> rows)
    {
        var views = new List<GroupView>();
        foreach (var row in rows)
            views.Add(new GroupView(Id(row, "id"), Text(row, "name"), Flag(row, "is_enabled")));
        views.Sort((left, right) => left.id.CompareTo(right.id));
        return views.ToArray();
    }

    static List<Dictionary<string, MappedValue>> Many(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Many(var items) ? items : [];

    static string Text(Dictionary<string, MappedValue> row, string name)
    {
        var value = Cell(row, name);
        return value is null or Value.Null ? "" : $"{value.ToObject()}";
    }

    static string Stamp(Dictionary<string, MappedValue> row, string name)
    {
        var value = Cell(row, name);
        return value is Value.DateTime ? value.IsoDateTime() : Text(row, name);
    }

    static bool Flag(Dictionary<string, MappedValue> row, string name) =>
        Cell(row, name) switch
        {
            Value.Bool(var flag) => flag,
            Value.Int(var number) => number != 0,
            _ => false
        };

    static int Id(Dictionary<string, MappedValue> row, string name)
    {
        var value = Cell(row, name);
        if (value is Value.Int(var number))
            return (int)number;
        var text = value is null or Value.Null ? "" : $"{value.ToObject()}";
        return int.TryParse(text, out var parsed) ? parsed : 0;
    }

    static Value? Cell(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var value) ? value : null;
}
