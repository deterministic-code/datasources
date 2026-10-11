namespace Datasources;

public sealed record BulkInsertStatement : IBulkInsertStatement
{
    public string Table { get; }
    public IReadOnlyList<string> Columns { get; }
    public IReadOnlyList<IReadOnlyList<Parameter>> Rows { get; }

    public BulkInsertStatement(string table, IReadOnlyList<string> columns, IReadOnlyList<IReadOnlyList<Parameter>> rows) =>
        (Table, Columns, Rows) = (table, columns, rows);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public IReadOnlyList<Parameter> Parameters => Flatten(Columns, Rows);

    internal static IReadOnlyList<Parameter> Flatten(
        IReadOnlyList<string> columns,
        IEnumerable<IReadOnlyList<Parameter>> rows)
    {
        var parameters = new List<Parameter>();
        var index = 0;
        foreach (var row in rows)
        {
            var byName = row.GroupBy(field => field.Name).ToDictionary(group => group.Key, group => group.Last());
            foreach (var column in columns)
                parameters.Add(new Parameter($"{column}__{index}",
                    byName.TryGetValue(column, out var field) ? field.Value : Value.Null.Instance));
            index++;
        }
        return parameters;
    }
}
