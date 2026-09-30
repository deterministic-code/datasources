namespace Datasources;

public sealed class CRUDRepositoryStatements : ReadonlyRepositoryStatements
{
    public CRUDRepositoryStatements(string table, IReadOnlyList<string> columns) : base(table, columns) { }

    public IInsertStatement Insert(IReadOnlyList<Parameter> fields) =>
        new InsertStatement(Table, fields);
    public IUpdateStatement Update(IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        new UpdateStatement(Table, fields, where);
    public IDeleteStatement Delete(IWhere? where = null) =>
        new DeleteStatement(Table, where);
    public SelectInsertedIdStatement SelectInsertedId(string column) => new(Table, column);
}
