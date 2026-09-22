namespace UserRolesSimpleCli;

public sealed record AddUserRole(string Username, string Role) : CliCommand
{
    public override Task<string> Run(App app) =>
        WriteLink.Execute(app, Username, "roles", "name", Role, "user_roles", Columns.Names.Map("role_id"), "add_user_role");
}
