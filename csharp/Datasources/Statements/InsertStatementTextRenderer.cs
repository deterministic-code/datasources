namespace Datasources;

public class InsertStatementTextRenderer
{
    public virtual string Render(InsertStatement statement) =>
        $"INSERT INTO {Quote(statement.Table)} ({string.Join(", ", statement.Fields.Select(field => Quote(field.Name)))}) VALUES ({string.Join(", ", statement.Fields.Select(field => Parameter(field.Name)))})";

    protected virtual string Quote(string name) => name;

    protected virtual string Parameter(string name) => $"${name}";
}
