namespace Datasources;

public sealed record DeleteStatement : IDeleteStatement
{
    readonly string? text;
    public string? Table { get; }
    public IWhere? Where { get; }

    public DeleteStatement(string table, IWhere? where = null) =>
        (Table, Where) = (table, where);

    DeleteStatement(string text, IWhere? where, bool _) => (this.text, Where) = (text, where);

    public static DeleteStatement Raw(string text, IWhere? where = null) => new(text, where, true);

    public string GetText() => text ?? WithWhere($"DELETE FROM {Table}", Where);

    static string WithWhere(string sql, IWhere? where) =>
        where is null ? sql : $"{sql} WHERE {where.GetText()}";
}
