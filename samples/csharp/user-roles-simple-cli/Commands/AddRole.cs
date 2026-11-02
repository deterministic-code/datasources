using Datasources;

namespace UserRolesSimpleCli;

public sealed record AddRole(string Name) : CliCommand
{
    public override async Task<string> Run(App app)
    {
        await WriteRow.Execute(
            app.Command,
            "roles",
            "add_role",
            [
                new Parameter("name", Name),
                new Parameter("is_enabled", true)
            ],
            [
                new Parameter("p_name", Name),
                new Parameter("p_is_enabled", true)
            ],
            app.Procedures);
        return "";
    }
}
