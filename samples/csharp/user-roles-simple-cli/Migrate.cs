using Datasources;

namespace UserRolesSimpleCli;

public static class Migrate
{
    public static async Task Apply(ICommand command)
    {
        foreach (var table in Schema.Tables(command.Connection.Provider, ColumnTypes.Map(command.Connection.Provider)))
            await command.Execute(IfNotExists(table.GetText()));
        if (command.Connection.Provider == "mysql")
            foreach (var sql in Schema.MysqlProcedures())
                await command.Execute(sql);
    }

    public static string IfNotExists(string sql) =>
        sql.StartsWith("CREATE TABLE ", StringComparison.Ordinal)
            ? string.Concat("CREATE TABLE IF NOT EXISTS ", sql.AsSpan("CREATE TABLE ".Length))
            : sql;
}
