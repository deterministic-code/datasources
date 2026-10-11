namespace Datasources;

public class BulkDeleteStatementTextRenderer
{
    public virtual string Render(BulkDeleteStatement statement)
    {
        if (statement.Ids.Count == 0)
            return $"DELETE FROM {statement.Table} WHERE 1 = 1";
        if (statement.Ids.Count == 1)
        {
            var column = statement.Ids[0];
            var names = string.Join(", ", statement.Rows.Select((_, index) => $"${column}__{index}"));
            return $"DELETE FROM {statement.Table} WHERE {column} IN ({names})";
        }
        var tuples = statement.Rows.Select((_, index) =>
            "(" + string.Join(", ", statement.Ids.Select(column => $"${column}__{index}")) + ")");
        return $"DELETE FROM {statement.Table} WHERE ({string.Join(", ", statement.Ids)}) IN ({string.Join(", ", tuples)})";
    }
}
