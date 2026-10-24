namespace Datasources;

public sealed class CRUDRepositoryBuilder<T> : ReadonlyRepositoryBuilder<T> where T : new()
{
    readonly RepositoryBuilderOptionsFactory<T> factory = new();
    public CRUDRepositoryStatements? CRUDStatements { get; private set; }

    public CRUDRepositoryBuilder<T> Add(CRUDRepositoryOptions<T> options)
    {
        factory.Add(options);
        return this;
    }

    public CRUDRepositoryBuilder<T> Add(string dialect, CRUDRepositoryOptions<T> options)
    {
        factory.Add(dialect, options);
        return this;
    }

    public CRUDRepositoryOptions<T> For(IConnection connection) => factory.For(connection.Provider);

    protected override IReadOnlyList<string> Ids(ReadonlyRepositoryMappingOptions<T> options) =>
        options is CRUDRepositoryOptions<T> crud ? crud.Ids : [];

    protected override ReadonlyRepositoryStatements Statements(string table, IReadOnlyList<string> columns)
    {
        CRUDStatements = new CRUDRepositoryStatements(table, columns, EntityMappers, PropertyMappers);
        return base.Statements(table, columns);
    }

    public CRUDRepository<T> Build(IConnection connection, CRUDRepositoryOptions<T> options) =>
        new(connection, options);
}
