namespace Datasources;

public class BulkUpdateStatementTextRenderer
{
    public virtual string Render(BulkUpdateStatement statement)
    {
        var assigns = string.Join(", ", statement.Set.Select(column => $"{column} = data.{column}"));
        return $"UPDATE {statement.Table} SET {assigns} FROM {Source(statement)} WHERE {On(statement)}";
    }

    protected static string On(BulkUpdateStatement statement) =>
        string.Join(" AND ", statement.Ids.Select(column => $"{statement.Table}.{column} = data.{column}"));

    protected string Source(BulkUpdateStatement statement)
    {
        var columns = statement.Set.Concat(statement.Ids).ToList();
        var selects = statement.Rows.Select((row, index) =>
        {
            var byName = row.Fields.Concat(row.Ids)
                .GroupBy(field => field.Name)
                .ToDictionary(group => group.Key, group => group.Last());
            return "SELECT " + string.Join(", ", columns.Select(column =>
                Cell(column, index, byName.TryGetValue(column, out var field) ? field.Value : Value.Null.Instance)));
        });
        return $"({string.Join(" UNION ALL ", selects)}) AS data";
    }

    protected virtual string Cell(string column, int index, Value value) =>
        $"${column}__{index} AS {column}";
}
