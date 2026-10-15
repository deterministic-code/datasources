using Datasources;
using Datasources.Sqlite;

namespace UserRolesSimpleCli;

public static class Migrate
{
    static readonly StatementTextRenderer Sqlite = new(text => new SqliteStatementRenderer(text));

    public static async Task Apply(ICommand command)
    {
        var provider = command.Connection.Provider;
        foreach (var table in Schema.Tables(provider))
            try
            {
                await (provider == "sqlite"
                    ? command.Execute(Native.N(IfNotExists(Sqlite.Render(table))))
                    : command.Execute(table));
            }
            catch (Exception ex) when (ex.Message.Contains("already exists", StringComparison.OrdinalIgnoreCase))
            {
            }
        if (command.Connection.Provider == "mysql")
            foreach (var sql in Schema.MysqlProcedures())
                await command.Execute(Native.N(sql));
    }

    public static string IfNotExists(string sql) =>
        sql.StartsWith("CREATE TABLE ", StringComparison.Ordinal)
            ? string.Concat("CREATE TABLE IF NOT EXISTS ", sql.AsSpan("CREATE TABLE ".Length))
            : sql;
}
