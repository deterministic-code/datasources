using Datasources;

namespace UserRolesSimpleCli;

public sealed record AddGroup(string Name) : CliCommand
{
    public override async Task<string> Run(App app)
    {
        var created = app.Now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = app.Token();
        var provider = app.Command.Connection.Provider;
        var column = Columns.Names.Map("name");
        await WriteRow.Execute(
            app.Command,
            Schema.Ident(provider, "groups"),
            "add_group",
            [
                new Parameter(column, Name),
                new Parameter("is_enabled", Schema.Flag(provider, true)),
                new Parameter("created", created),
                new Parameter("updated", updated)
            ],
            [
                new Parameter("p_name", Name),
                new Parameter("p_is_enabled", true),
                new Parameter("p_created", created),
                new Parameter("p_updated", updated)
            ]);
        return "";
    }
}
