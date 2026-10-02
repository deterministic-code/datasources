namespace Datasources;

public class ReturnStatementTextRenderer
{
    public virtual string Render(ReturnStatement statement) =>
        $"SELECT {statement.Name}({Args(statement.Parameters)})";

    static string Args(IReadOnlyList<Parameter> parameters) =>
        string.Join(", ", parameters.Where(p => p.Direction != ParameterDirection.ReturnValue).Select(p => $"${p.Name}"));
}
