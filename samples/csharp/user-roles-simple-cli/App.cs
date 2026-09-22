using System.Text.Json;
using Datasources;

namespace UserRolesSimpleCli;

public sealed class App
{
    readonly ICommand command;
    readonly Func<DateTime> now;
    readonly Func<string, string> hash;
    readonly Func<byte[]> token;

    public App(
        ICommand command,
        Func<DateTime>? now = null,
        Func<string, string>? hash = null,
        Func<byte[]>? token = null)
    {
        this.command = command;
        this.now = now ?? (() => DateTime.UtcNow);
        this.hash = hash ?? PasswordHash.Hash;
        this.token = token ?? (() => Guid.NewGuid().ToByteArray());
    }

    public Task<string> Run(CliCommand parsed) => parsed switch
    {
        CliCommand.AddUser add => WriteUser(add.Username, add.Password),
        CliCommand.AddRole add => WriteNamed("roles", "name", add.Name, "add_role"),
        CliCommand.AddGroup add => WriteNamed("groups", Columns.Names.Map("name"), add.Name, "add_group"),
        CliCommand.AddUserRole add => WriteLink(add.Username, "roles", "name", add.Role, "user_roles", Columns.Names.Map("role_id"), "add_user_role"),
        CliCommand.AddUserGroup add => WriteLink(add.Username, "groups", Columns.Names.Map("name"), add.Group, "user_groups", "group_id", "add_user_group"),
        CliCommand.EnableUser enable => SetEnabled(enable.Username, true),
        CliCommand.DisableUser disable => SetEnabled(disable.Username, false),
        CliCommand.ListUsersRolesGroups => List(),
        _ => throw new InvalidOperationException("empty command")
    };

    async Task<string> WriteUser(string username, string password)
    {
        var hashed = hash(password);
        var created = now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = token();
        await Execute(
            Table("users"),
            "add_user",
            Fields(
                new Parameter("username", username),
                new Parameter("password", hashed),
                new Parameter("is_enabled", Flag(true)),
                new Parameter("created", created),
                new Parameter("updated", updated)),
            Fields(
                new Parameter("p_username", username),
                new Parameter("p_password", hashed),
                new Parameter("p_is_enabled", true),
                new Parameter("p_created", created),
                new Parameter("p_updated", updated)));
        return "";
    }

    async Task<string> WriteNamed(string table, string column, string value, string procedure)
    {
        var created = now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = token();
        await Execute(
            Table(table),
            procedure,
            Fields(
                new Parameter(column, value),
                new Parameter("is_enabled", Flag(true)),
                new Parameter("created", created),
                new Parameter("updated", updated)),
            Fields(
                new Parameter("p_name", value),
                new Parameter("p_is_enabled", true),
                new Parameter("p_created", created),
                new Parameter("p_updated", updated)));
        return "";
    }

    async Task<string> WriteLink(
        string username,
        string rightTable,
        string rightColumn,
        string rightValue,
        string linkTable,
        string linkColumn,
        string procedure)
    {
        var userId = await Id(Table("users"), "username", username);
        var rightId = await Id(Table(rightTable), rightColumn, rightValue);
        var created = now().ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'");
        var updated = token();
        var insert = Fields(
            new Parameter("user_id", userId),
            new Parameter(linkColumn, rightId),
            new Parameter("created", created),
            new Parameter("updated", updated));
        var call = Fields(
            new Parameter("p_user_id", userId),
            new Parameter(linkColumn == "bad_role_id" ? "p_role_id" : "p_group_id", rightId),
            new Parameter("p_created", created),
            new Parameter("p_updated", updated));
        if (Mysql)
            await command.Execute(new ExecuteStoredProcedureStatement(procedure, call), call);
        else
            await command.Execute(new InsertStatement(Table(linkTable), insert), insert);
        return "";
    }

    async Task<string> SetEnabled(string username, bool enabled)
    {
        var updated = token();
        if (Mysql)
        {
            var fields = Fields(
                new Parameter("p_username", username),
                new Parameter("p_is_enabled", Flag(enabled)),
                new Parameter("p_updated", updated));
            await command.Execute(new ExecuteStoredProcedureStatement("set_user_enabled", fields), fields);
            return "";
        }
        var id = await Id(Table("users"), "username", username);
        var fieldsSql = Fields(new Parameter("is_enabled", Flag(enabled)), new Parameter("updated", updated));
        var where = new WhereClause(new Datasources.Operator.Equals("id", id));
        await command.Execute(new UpdateStatement(Table("users"), fieldsSql, where), [.. fieldsSql, .. where.Parameters]);
        return "";
    }

    async Task<string> List()
    {
        var reader = await command.ExecuteReader(UsersRolesGroupsQuery.Create(command.Connection.Provider));
        var rows = new List<Row>();
        while (await reader.Next())
            if (reader.Current is { } row)
                rows.Add(row);
        return JsonSerializer.Serialize(AsUserView.FromMapped(new FlatToNestedMapper(UsersView.TypeMap).Map(rows)));
    }

    async Task<int> Id(string table, string column, string value)
    {
        var where = new WhereClause(new Datasources.Operator.Equals(column, value));
        var reader = await command.ExecuteReader(new QueryStatement(table, ["id"], where), where.Parameters);
        return await reader.Next() && reader.Current is { } row
            ? row["id"] switch
            {
                Value.Int(var number) => (int)number,
                _ => throw new InvalidOperationException($"{table} '{value}' not found")
            }
            : throw new InvalidOperationException($"{table} '{value}' not found");
    }

    Task Execute(string table, string procedure, IReadOnlyList<Parameter> insert, IReadOnlyList<Parameter> call) =>
        Mysql
            ? command.Execute(new ExecuteStoredProcedureStatement(procedure, call), call)
            : command.Execute(new InsertStatement(table, insert), insert);

    bool Mysql => command.Connection.Provider == "mysql";

    string Table(string name) => Schema.Ident(command.Connection.Provider, name);

    Value Flag(bool value) => command.Connection.Provider == "sqlite" ? value ? 1 : 0 : value;

    static Parameter[] Fields(params Parameter[] fields) => fields;
}
