namespace Datasources;

public class BulkDeleteStatementTextRenderer
{
    public virtual string Render(BulkDeleteStatement statement)
    {
        if (statement.Ids.Count == 0)
            return $"DELETE FROM {Quote(statement.Table)} WHERE 1 = 1";
        if (statement.Ids.Count == 1)
        {
            var column = statement.Ids[0];
            var names = string.Join(", ", statement.Rows.Select((_, index) => Parameter($"{column}__{index}")));
            return $"DELETE FROM {Quote(statement.Table)} WHERE {Quote(column)} IN ({names})";
        }
        var tuples = statement.Rows.Select((_, index) =>
            "(" + string.Join(", ", statement.Ids.Select(column => Parameter($"{column}__{index}"))) + ")");
        return $"DELETE FROM {Quote(statement.Table)} WHERE ({string.Join(", ", statement.Ids.Select(Quote))}) IN ({string.Join(", ", tuples)})";
    }

    protected virtual string Quote(string name) => name;

    protected virtual string Parameter(string name) => $"${name}";
}
