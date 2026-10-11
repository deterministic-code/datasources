namespace Datasources;

public class BulkInsertStatementTextRenderer
{
    public virtual string Render(BulkInsertStatement statement)
    {
        var values = statement.Rows.Select((_, index) =>
            "(" + string.Join(", ", statement.Columns.Select(column => $"${column}__{index}")) + ")");
        return $"INSERT INTO {statement.Table} ({string.Join(", ", statement.Columns)}) VALUES {string.Join(", ", values)}";
    }
}
