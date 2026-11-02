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
        var userId = await LookupId.Of(app.Command, "users", "username", username);
        var rightId = await LookupId.Of(app.Command, rightTable, rightColumn, rightValue);
        Parameter[] insert =
        [
            new("user_id", userId),
            new(linkColumn, rightId)
        ];
        Parameter[] call =
        [
            new("p_user_id", userId),
            new(linkColumn == "bad_role_id" ? "p_role_id" : "p_group_id", rightId)
        ];
        await WriteRow.Execute(app.Command, linkTable, procedure, insert, call, app.Procedures);
        return "";
    }
}
