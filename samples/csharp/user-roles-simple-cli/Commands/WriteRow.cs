using Datasources;

namespace UserRolesSimpleCli;

public static class WriteRow
{
    public static Task Execute(ICommand command, string table, string procedure, IReadOnlyList<Parameter> insert, IReadOnlyList<Parameter> call) =>
        command.Connection.Provider == "mysql"
            ? command.Execute(new ExecuteStoredProcedureStatement(procedure, call), call)
            : command.Execute(new InsertStatement(table, insert), insert);
}
