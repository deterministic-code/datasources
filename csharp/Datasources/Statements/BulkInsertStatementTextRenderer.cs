namespace Datasources;

public class BulkInsertStatementTextRenderer
{
    public virtual string Render(BulkInsertStatement statement)
    {
        var values = statement.Rows.Select((_, index) =>
            "(" + string.Join(", ", statement.Columns.Select(column => Parameter($"{column}__{index}"))) + ")");
        return $"INSERT INTO {Quote(statement.Table)} ({string.Join(", ", statement.Columns.Select(Quote))}) VALUES {string.Join(", ", values)}";
    }

    protected virtual string Quote(string name) => name;

    protected virtual string Parameter(string name) => $"${name}";
}
