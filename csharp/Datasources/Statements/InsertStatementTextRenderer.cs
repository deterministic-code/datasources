namespace Datasources;

public class InsertStatementTextRenderer
{
    public virtual string Render(InsertStatement statement) =>
        $"INSERT INTO {statement.Table} ({string.Join(", ", statement.Fields.Select(field => field.Name))}) VALUES ({string.Join(", ", statement.Fields.Select(field => $"${field.Name}"))})";
}
