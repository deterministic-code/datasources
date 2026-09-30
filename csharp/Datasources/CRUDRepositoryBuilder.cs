namespace Datasources;

public sealed class CRUDRepositoryBuilder<T> : ReadonlyRepositoryBuilder<T> where T : new()
{
    protected override IReadOnlyList<string> Ids(ReadonlyRepositoryOptions<T> options) =>
        options is CRUDRepositoryOptions<T> crud ? crud.Ids : [];

    protected override ReadonlyRepositoryStatements Statements(string table, IReadOnlyList<string> columns) =>
        new CRUDRepositoryStatements(table, columns, EntityMappers, PropertyMappers);

    public CRUDRepository<T> Build(IConnection connection, CRUDRepositoryOptions<T> options) =>
        new(connection, options);
}
