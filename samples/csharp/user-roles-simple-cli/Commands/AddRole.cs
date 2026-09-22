namespace UserRolesSimpleCli;

public sealed record AddRole(string Name) : CliCommand
{
    public override Task<string> Run(App app) => app.WriteNamed("roles", "name", Name, "add_role");
}
