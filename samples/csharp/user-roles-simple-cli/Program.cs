using System.Text.Json;
using Datasources;

namespace UserRolesSimpleCli;

public static class Program
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

    public static readonly TransformMapper<string, string> Names = new(
        name => name switch
        {
            "name" => "ugly_name",
            "role_id" => "bad_role_id",
            _ => name
        },
        name => name switch
        {
            "ugly_name" => "name",
            "bad_role_id" => "role_id",
            _ => name
        });

    public static TypeMap UsersView { get; } = new()
    {
        Key = "id",
        Fields = new Dictionary<string, TypeMapField>
        {
            ["id"] = "id",
            ["username"] = "username",
            ["is_enabled"] = "is_enabled",
            ["created"] = "created",
            ["roles"] = new TypeMapGroup
            {
                Key = "role_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "role_id",
                    ["name"] = "role_name",
                    ["is_enabled"] = "role_is_enabled"
                }
            },
            ["groups"] = new TypeMapGroup
            {
                Key = "group_id",
                Fields = new Dictionary<string, TypeMapField>
                {
                    ["id"] = "group_id",
                    ["name"] = "group_name",
                    ["is_enabled"] = "group_is_enabled"
                }
            }
        }
    };

    public static ISelectStatement ListQuery() =>
        new SelectStatement(new()
        {
            Table = "users",
            Fields =
            [
                "users.id AS id",
                "users.username AS username",
                "users.is_enabled AS is_enabled",
                "users.created AS created",
                "roles.id AS role_id",
                "roles.name AS role_name",
                "roles.is_enabled AS role_is_enabled",
                "groups.id AS group_id",
                "groups.ugly_name AS group_name",
                "groups.is_enabled AS group_is_enabled"
            ],
            Joins =
            [
                Join.Left("user_roles", "users.id", "user_roles.user_id"),
                Join.Left("roles", "user_roles.bad_role_id", "roles.id"),
                Join.Left("user_groups", "users.id", "user_groups.user_id"),
                Join.Left("groups", "user_groups.group_id", "groups.id")
            ],
            OrderBy = [OrderBy.Asc("users.id"), OrderBy.Asc("roles.id"), OrderBy.Asc("groups.id")],
        });

    public static string Hash(string password) => BCrypt.Net.BCrypt.HashPassword(password, 4);

    public static (string Verb, string[] Values) Parsed(string[] args) =>
        args.Length == 0
            ? throw new InvalidOperationException(Usage)
            : args[0] switch
            {
                "add-user" when args.Length == 3 => ("add-user", [args[1], args[2]]),
                "add-user" => throw new InvalidOperationException("Usage: add-user <username> <password>"),
                "add-role" when args.Length == 2 => ("add-role", [args[1]]),
                "add-role" => throw new InvalidOperationException("Usage: add-role <name>"),
                "add-group" when args.Length == 2 => ("add-group", [args[1]]),
                "add-group" => throw new InvalidOperationException("Usage: add-group <name>"),
                "add-user-role" when args.Length == 3 => ("add-user-role", [args[1], args[2]]),
                "add-user-role" => throw new InvalidOperationException("Usage: add-user-role <username> <role>"),
                "add-user-group" when args.Length == 3 => ("add-user-group", [args[1], args[2]]),
                "add-user-group" => throw new InvalidOperationException("Usage: add-user-group <username> <group>"),
                "enable-user" when args.Length == 2 => ("enable-user", [args[1]]),
                "enable-user" => throw new InvalidOperationException("Usage: enable-user <username>"),
                "disable-user" when args.Length == 2 => ("disable-user", [args[1]]),
                "disable-user" => throw new InvalidOperationException("Usage: disable-user <username>"),
                "list-users-roles-groups" when args.Length == 1 => ("list-users-roles-groups", []),
                var verb => throw new InvalidOperationException($"Unknown command: {verb}\n{Usage}")
            };

    public static async Task<int> Main(string[] args)
    {
        try
        {
            await using var ds = Datasource.Default;
            var connection = await ds.CreateConnection();
            await connection.Open();
            var command = await ds.CreateCommand(connection);
            var stamp = Env("CLI_NOW");
            command.AddMiddleware(new ConsoleCommandMiddleware(
                Console.Error.WriteLine,
                stamp is null ? null : () => DateTimeOffset.Parse(stamp, null, System.Globalization.DateTimeStyles.RoundtripKind)));
            Console.Write(await Run(command, args));
            return 0;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine(ex.Message);
            return 1;
        }
    }

    public static async Task<string> Run(
        ICommand command,
        string[] args,
        Func<string, string>? hash = null,
        bool procedures = true)
    {
        var parsed = Parsed(args);
        hash ??= Hash;
        return parsed.Verb switch
        {
            "add-user" => await Write(
                command,
                "users",
                "add_user",
                [
                    new Parameter("username", parsed.Values[0]),
                    new Parameter("password", hash(parsed.Values[1])),
                    new Parameter("is_enabled", true),
                    Created()
                ],
                [
                    new Parameter("p_username", parsed.Values[0]),
                    new Parameter("p_password", hash(parsed.Values[1])),
                    new Parameter("p_is_enabled", true),
                    new Parameter("p_created", Created().Value),
                    Updated("p_updated")
                ],
                procedures),
            "add-role" => await Write(
                command,
                "roles",
                "add_role",
                [new Parameter("name", parsed.Values[0]), new Parameter("is_enabled", true), Created()],
                [new Parameter("p_name", parsed.Values[0]), new Parameter("p_is_enabled", true), new Parameter("p_created", Created().Value), Updated("p_updated")],
                procedures),
            "add-group" => await Write(
                command,
                "groups",
                "add_group",
                [new Parameter(Names.Map("name"), parsed.Values[0]), new Parameter("is_enabled", true), Created()],
                [new Parameter("p_name", parsed.Values[0]), new Parameter("p_is_enabled", true), new Parameter("p_created", Created().Value), Updated("p_updated")],
                procedures),
            "add-user-role" => await Link(command, parsed.Values[0], "roles", "name", parsed.Values[1], "user_roles", Names.Map("role_id"), "add_user_role", procedures),
            "add-user-group" => await Link(command, parsed.Values[0], "groups", Names.Map("name"), parsed.Values[1], "user_groups", "group_id", "add_user_group", procedures),
            "enable-user" => await SetEnabled(command, parsed.Values[0], true, procedures),
            "disable-user" => await SetEnabled(command, parsed.Values[0], false, procedures),
            "list-users-roles-groups" => await List(command),
            _ => throw new InvalidOperationException("empty command")
        };
    }

    static async Task<string> Write(
        ICommand command,
        string table,
        string procedure,
        IReadOnlyList<Parameter> insert,
        IReadOnlyList<Parameter> call,
        bool procedures)
    {
        if (procedures && command.Connection.Provider == "mysql")
            await command.Execute(new ExecuteStoredProcedureStatement(procedure, call), call);
        else
            await command.Execute(new InsertStatement(table, insert), insert);
        return "";
    }

    static async Task<string> Link(
        ICommand command,
        string username,
        string rightTable,
        string rightColumn,
        string rightValue,
        string linkTable,
        string linkColumn,
        string procedure,
        bool procedures)
    {
        var userId = await LookupId(command, "users", "username", username);
        var rightId = await LookupId(command, rightTable, rightColumn, rightValue);
        return await Write(
            command,
            linkTable,
            procedure,
            [new Parameter("user_id", userId), new Parameter(linkColumn, rightId), Created()],
            [
                new Parameter("p_user_id", userId),
                new Parameter(linkColumn == "bad_role_id" ? "p_role_id" : "p_group_id", rightId),
                new Parameter("p_created", Created().Value),
                Updated("p_updated")
            ],
            procedures);
    }

    static async Task<string> SetEnabled(ICommand command, string username, bool enabled, bool procedures)
    {
        if (procedures && command.Connection.Provider == "mysql")
        {
            Parameter[] fields = [new("p_username", username), new("p_is_enabled", enabled), Updated("p_updated")];
            await command.Execute(new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
            return "";
        }
        var id = await LookupId(command, "users", "username", username);
        Parameter[] fieldsSql = [new("is_enabled", enabled)];
        var where = new Where(new Datasources.Operator.Equals("id", id));
        await command.Execute(new UpdateStatement("users", fieldsSql, where), [.. fieldsSql, .. where.Parameters]);
        return "";
    }

    static async Task<int> LookupId(ICommand command, string table, string column, string value)
    {
        var where = new Where(new Datasources.Operator.Equals(column, value));
        await using var reader = await command.ExecuteReader(new SelectStatement(new() { Table = table, Fields = ["id"], Where = where }), where.Parameters);
        return await reader.Next() && reader.Current is { } row
            ? row["id"] switch
            {
                Value.Int(var number) => (int)number,
                _ => throw new InvalidOperationException($"{table} '{value}' not found")
            }
            : throw new InvalidOperationException($"{table} '{value}' not found");
    }

    static async Task<string> List(ICommand command)
    {
        await using var reader = await command.ExecuteReader(ListQuery());
        var rows = new List<Row>();
        while (await reader.Next())
            if (reader.Current is { } row)
                rows.Add(row);
        return JsonSerializer.Serialize(new UserViewMapper().Map(new FlatToNestedMapper(UsersView).Map(rows)));
    }

    static Parameter Created() => new("created", DateTime.UtcNow);

    static Parameter Updated(string name) => new(name, new byte[] { 0 });

    static string? Env(string key)
    {
        var value = Environment.GetEnvironmentVariable(key);
        if (!string.IsNullOrWhiteSpace(value)) return value.Trim();
        if (!File.Exists(".env")) return null;
        foreach (var line in File.ReadAllLines(".env"))
        {
            var i = line.IndexOf('=');
            if (i > 0 && line[..i].Trim() == key)
            {
                var raw = line[(i + 1)..].Trim();
                return string.IsNullOrWhiteSpace(raw) ? null : raw;
            }
        }
        return null;
    }
}

public sealed record UserView(int id, string username, bool is_enabled, string created, RoleView[] roles, GroupView[] groups);

public sealed record RoleView(int id, string name, bool is_enabled);

public sealed record GroupView(int id, string name, bool is_enabled);

public sealed class UserViewMapper : IMapper<IEnumerable<Dictionary<string, MappedValue>>, IReadOnlyList<UserView>>
{
    public IReadOnlyList<UserView> Map(IEnumerable<Dictionary<string, MappedValue>> rows)
    {
        var users = new List<UserView>();
        foreach (var row in rows)
            users.Add(new UserView(
                Id(row, "id"),
                Text(row, "username"),
                Flag(row, "is_enabled"),
                Text(row, "created"),
                Roles(Many(row, "roles")),
                Groups(Many(row, "groups"))));
        users.Sort((left, right) => left.id.CompareTo(right.id));
        return users;
    }

    public IEnumerable<Dictionary<string, MappedValue>> ReverseMap(IReadOnlyList<UserView> value) =>
        throw new NotSupportedException();

    static RoleView[] Roles(List<Dictionary<string, MappedValue>> rows)
    {
        var views = new List<RoleView>();
        foreach (var row in rows)
            views.Add(new RoleView(Id(row, "id"), Text(row, "name"), Flag(row, "is_enabled")));
        views.Sort((left, right) => left.id.CompareTo(right.id));
        return views.ToArray();
    }

    static GroupView[] Groups(List<Dictionary<string, MappedValue>> rows)
    {
        var views = new List<GroupView>();
        foreach (var row in rows)
            views.Add(new GroupView(Id(row, "id"), Text(row, "name"), Flag(row, "is_enabled")));
        views.Sort((left, right) => left.id.CompareTo(right.id));
        return views.ToArray();
    }

    static List<Dictionary<string, MappedValue>> Many(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Many(var items) ? items : [];

    static string Text(Dictionary<string, MappedValue> row, string name)
    {
        var value = Cell(row, name);
        if (value is Value.DateTime) return value.IsoDateTime();
        return value is null or Value.Null ? "" : $"{value.ToObject()}";
    }

    static bool Flag(Dictionary<string, MappedValue> row, string name) =>
        Cell(row, name) switch
        {
            Value.Bool(var flag) => flag,
            Value.Int(var number) => number != 0,
            _ => false
        };

    static int Id(Dictionary<string, MappedValue> row, string name)
    {
        var value = Cell(row, name);
        if (value is Value.Int(var number))
            return (int)number;
        var text = value is null or Value.Null ? "" : $"{value.ToObject()}";
        return int.TryParse(text, out var parsed) ? parsed : 0;
    }

    static Value? Cell(Dictionary<string, MappedValue> row, string name) =>
        row.TryGetValue(name, out var mapped) && mapped is MappedValue.Cell(var value) ? value : null;
}
