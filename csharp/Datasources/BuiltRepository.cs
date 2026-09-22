namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<T, Row> RowConverter,
    CRUDMapper<T> CRUDMapper,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids) where T : new();
