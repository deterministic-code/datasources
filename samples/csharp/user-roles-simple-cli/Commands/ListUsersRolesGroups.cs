using System.Text.Json;
using Datasources;

namespace UserRolesSimpleCli;

public sealed record ListUsersRolesGroups : CliCommand
{
    public override async Task<string> Run(App app)
    {
        await using var reader = await app.Command.ExecuteReader(UsersRolesGroupsQuery.Create(app.Command.Connection.Provider));
        var rows = new List<Row>();
        while (await reader.Next())
            if (reader.Current is { } row)
                rows.Add(row);
        return JsonSerializer.Serialize(AsUserView.FromMapped(new FlatToNestedMapper(UsersView.TypeMap).Map(rows)));
    }
}
