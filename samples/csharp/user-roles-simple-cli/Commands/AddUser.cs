using Datasources;

namespace UserRolesSimpleCli;

public sealed record AddUser(string Username, string Password) : CliCommand
{
    public override async Task<string> Run(App app)
    {
        var hashed = app.Hash(Password);
        var created = app.Now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = app.Token();
        var provider = app.Command.Connection.Provider;
        await WriteRow.Execute(
            app.Command,
            Schema.Ident(provider, "users"),
            "add_user",
            [
                new Parameter("username", Username),
                new Parameter("password", hashed),
                new Parameter("is_enabled", Schema.Flag(provider, true)),
                new Parameter("created", created),
                new Parameter("updated", updated)
            ],
            [
                new Parameter("p_username", Username),
                new Parameter("p_password", hashed),
                new Parameter("p_is_enabled", true),
                new Parameter("p_created", created),
                new Parameter("p_updated", updated)
            ]);
        return "";
    }
}
