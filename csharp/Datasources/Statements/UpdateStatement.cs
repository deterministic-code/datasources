namespace Datasources;

public sealed record UpdateStatement : IUpdateStatement
{
    readonly string text;
    public UpdateStatement(string text, IWhere? where = null) => (this.text, Where) = (text, where);
    public UpdateStatement(string table, IReadOnlyList<Parameter> fields, IWhere? where = null)
        : this(WithWhere($"UPDATE {table} SET {string.Join(", ", fields.Select(field => $"{field.Name} = ${field.Name}"))}", where), where) { }
    public IWhere? Where { get; }
    public string GetText() => text;

    static string WithWhere(string sql, IWhere? where) =>
        where is null ? sql : $"{sql} WHERE {where.GetText()}";
}
