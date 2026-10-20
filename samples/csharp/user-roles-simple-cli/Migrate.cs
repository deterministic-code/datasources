using Datasources;

namespace UserRolesSimpleCli;

public static class Migrate
{
    public static async Task Apply(ICommand command)
    {
        foreach (var table in Schema.Tables(command.Connection.Provider))
            await command.Execute(table);
        if (command.Connection.Provider == "mysql")
            foreach (var sql in Schema.MysqlProcedures())
                await command.Execute(Native.N(sql));
    }
}
