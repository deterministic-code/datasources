using Datasources;

namespace UserRolesSimpleCli;

public static class WriteLink
{
    public static async Task<string> Execute(
        App app,
        string username,
        string rightTable,
        string rightColumn,
        string rightValue,
        string linkTable,
        string linkColumn,
        string procedure)
    {
        var provider = app.Command.Connection.Provider;
        var userId = await LookupId.Of(app.Command, Schema.Ident(provider, "users"), "username", username);
        var rightId = await LookupId.Of(app.Command, Schema.Ident(provider, rightTable), rightColumn, rightValue);
        var created = app.Now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = app.Token();
        Parameter[] insert =
        [
            new("user_id", userId),
            new(linkColumn, rightId),
            new("created", created),
            new("updated", updated)
        ];
        Parameter[] call =
        [
            new("p_user_id", userId),
            new(linkColumn == "bad_role_id" ? "p_role_id" : "p_group_id", rightId),
            new("p_created", created),
            new("p_updated", updated)
        ];
        await WriteRow.Execute(app.Command, Schema.Ident(provider, linkTable), procedure, insert, call);
        return "";
    }
}
