namespace Datasources;

public class ReadonlyRepositoryStatements
{
    readonly string table;
    readonly IReadOnlyList<string> columns;

    public ReadonlyRepositoryStatements(string table, IReadOnlyList<string> columns) =>
        (this.table, this.columns) = (table, columns);

    protected string Table => table;
    public IReadOnlyList<string> ReadColumns => columns;

    public ISelectStatement Select(IWhere? where = null) =>
        new SelectStatement(new() { Table = table, Fields = columns, Where = where });
}
