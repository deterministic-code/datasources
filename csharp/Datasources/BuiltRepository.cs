namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<T, Row> RowConverter,
    IMapper<T, T> RowsMapper,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids,
    IMapper<Value, T>? Create) where T : new();
