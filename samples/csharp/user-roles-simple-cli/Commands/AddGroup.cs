namespace UserRolesSimpleCli;

public sealed record AddGroup(string Name) : CliCommand
{
    public override Task<string> Run(App app) =>
        app.WriteNamed("groups", Columns.Names.Map("name"), Name, "add_group");
}
