using Datasources;

namespace UserRolesSimpleCli;

public static class SetUserEnabled
{
    public static async Task<string> Execute(App app, string username, bool enabled)
    {
        var provider = app.Command.Connection.Provider;
        if (app.Procedures && provider == "mysql")
        {
            Parameter[] fields =
            [
                new("p_username", username),
                new("p_is_enabled", enabled)
            ];
            await app.Command.Execute(new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
            return "";
        }
        var id = await LookupId.Of(app.Command, "users", "username", username);
        Parameter[] fieldsSql = [new("is_enabled", enabled)];
        var where = new Where(new Datasources.Operator.Equals("id", id));
        await app.Command.Execute(new UpdateStatement("users", fieldsSql, where), [.. fieldsSql, .. where.Parameters]);
        return "";
    }
}
