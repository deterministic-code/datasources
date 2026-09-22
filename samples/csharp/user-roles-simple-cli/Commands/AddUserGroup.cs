namespace UserRolesSimpleCli;

public sealed record AddUserGroup(string Username, string Group) : CliCommand
{
    public override Task<string> Run(App app) =>
        app.WriteLink(Username, "groups", Columns.Names.Map("name"), Group, "user_groups", "group_id", "add_user_group");
}
