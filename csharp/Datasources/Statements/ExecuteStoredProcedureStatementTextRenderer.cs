namespace Datasources;

public class ExecuteStoredProcedureStatementTextRenderer
{
    public virtual string Render(ExecuteStoredProcedureStatement statement) =>
        $"CALL {statement.Name}({Args(statement.Parameters)})";

    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
