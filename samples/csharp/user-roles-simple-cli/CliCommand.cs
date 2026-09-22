namespace UserRolesSimpleCli;

public abstract record CliCommand
{
    public sealed record AddUser(string Username, string Password) : CliCommand;
    public sealed record AddRole(string Name) : CliCommand;
    public sealed record AddGroup(string Name) : CliCommand;
    public sealed record AddUserRole(string Username, string Role) : CliCommand;
    public sealed record AddUserGroup(string Username, string Group) : CliCommand;
    public sealed record EnableUser(string Username) : CliCommand;
    public sealed record DisableUser(string Username) : CliCommand;
    public sealed record ListUsersRolesGroups : CliCommand;
}
