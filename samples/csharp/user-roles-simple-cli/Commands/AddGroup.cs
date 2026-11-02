using Datasources;

namespace UserRolesSimpleCli;

public sealed record AddGroup(string Name) : CliCommand
{
    public override async Task<string> Run(App app)
    {
        var column = Columns.Names.Map("name");
        await WriteRow.Execute(
            app.Command,
            "groups",
            "add_group",
            [
                new Parameter(column, Name),
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
