using Datasources;

namespace UserRolesSimpleCli;

public static class SetUserEnabled
{
    public static async Task<string> Execute(App app, string username, bool enabled)
    {
        var provider = app.Command.Connection.Provider;
        var updated = app.Token();
        if (app.Procedures && provider == "mysql")
        {
            Parameter[] fields =
            [
                new("p_username", username),
                new("p_is_enabled", Schema.Flag(provider, enabled)),
                new("p_updated", updated)
            ];
            await app.Command.Execute(new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
            return "";
        }
        var id = await LookupId.Of(app.Command, Schema.Ident(provider, "users"), "username", username);
        Parameter[] fieldsSql = [new("is_enabled", Schema.Flag(provider, enabled)), new("updated", updated)];
        var where = new Where(new Datasources.Operator.Equals("id", id));
        await app.Command.Execute(new UpdateStatement(Schema.Ident(provider, "users"), fieldsSql, where), [.. fieldsSql, .. where.Parameters]);
        return "";
    }
}
