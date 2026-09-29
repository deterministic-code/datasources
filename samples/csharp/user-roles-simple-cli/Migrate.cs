using Datasources;

namespace UserRolesSimpleCli;

public static class Migrate
{
    public static async Task Apply(ICommand command)
    {
        var provider = command.Connection.Provider;
        foreach (var table in Schema.Tables(provider))
            await (provider == "sqlite"
                ? command.Execute(IfNotExists(table.GetText()))
                : command.Execute(table));
        if (command.Connection.Provider == "mysql")
            foreach (var sql in Schema.MysqlProcedures())
                await command.Execute(sql);
    }

    public static string IfNotExists(string sql) =>
        sql.StartsWith("CREATE TABLE ", StringComparison.Ordinal)
            ? string.Concat("CREATE TABLE IF NOT EXISTS ", sql.AsSpan("CREATE TABLE ".Length))
            : sql;
}
