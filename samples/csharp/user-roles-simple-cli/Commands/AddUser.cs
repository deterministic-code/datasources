using Datasources;

namespace UserRolesSimpleCli;

public sealed record AddUser(string Username, string Password) : CliCommand
{
    public override async Task<string> Run(App app)
    {
        var hashed = app.Hash(Password);
        await WriteRow.Execute(
            app.Command,
            "users",
            "add_user",
            [
                new Parameter("username", Username),
                new Parameter("password", hashed),
                new Parameter("is_enabled", true)
            ],
            [
                new Parameter("p_username", Username),
                new Parameter("p_password", hashed),
                new Parameter("p_is_enabled", true)
            ],
            app.Procedures);
        return "";
    }
}
