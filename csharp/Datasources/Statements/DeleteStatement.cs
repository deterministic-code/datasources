namespace Datasources;

public sealed record DeleteStatement : IDeleteStatement
{
    readonly string text;
    public DeleteStatement(string table, IWhere? where = null) =>
        (this.text, Where) = (WithWhere($"DELETE FROM {table}", where), where);
    DeleteStatement(string text, IWhere? where, bool _) => (this.text, Where) = (text, where);
    public static DeleteStatement Raw(string text, IWhere? where = null) => new(text, where, true);
    public IWhere? Where { get; }
    public string GetText() => text;

    static string WithWhere(string sql, IWhere? where) =>
        where is null ? sql : $"{sql} WHERE {where.GetText()}";
}
