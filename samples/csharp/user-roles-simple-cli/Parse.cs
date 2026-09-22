namespace UserRolesSimpleCli;

public static class Parse
{
    public const string Usage =
        """
        add-user <username> <password>
        add-role <name>
        add-group <name>
        add-user-role <username> <role>
        add-user-group <username> <group>
        enable-user <username>
        disable-user <username>
        list-users-roles-groups
        """;

    public static CliCommand Args(string[] args) =>
        args.Length == 0
            ? throw new InvalidOperationException(Usage)
            : args[0] switch
            {
                "add-user" when args.Length == 3 => new AddUser(args[1], args[2]),
                "add-user" => throw new InvalidOperationException("Usage: add-user <username> <password>"),
                "add-role" when args.Length == 2 => new AddRole(args[1]),
                "add-role" => throw new InvalidOperationException("Usage: add-role <name>"),
                "add-group" when args.Length == 2 => new AddGroup(args[1]),
                "add-group" => throw new InvalidOperationException("Usage: add-group <name>"),
                "add-user-role" when args.Length == 3 => new AddUserRole(args[1], args[2]),
                "add-user-role" => throw new InvalidOperationException("Usage: add-user-role <username> <role>"),
                "add-user-group" when args.Length == 3 => new AddUserGroup(args[1], args[2]),
                "add-user-group" => throw new InvalidOperationException("Usage: add-user-group <username> <group>"),
                "enable-user" when args.Length == 2 => new EnableUser(args[1]),
                "enable-user" => throw new InvalidOperationException("Usage: enable-user <username>"),
                "disable-user" when args.Length == 2 => new DisableUser(args[1]),
                "disable-user" => throw new InvalidOperationException("Usage: disable-user <username>"),
                "list-users-roles-groups" when args.Length == 1 => new ListUsersRolesGroups(),
                var verb => throw new InvalidOperationException($"Unknown command: {verb}\n{Usage}")
            };
}
