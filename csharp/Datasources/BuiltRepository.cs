namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<Row, Row>? RowConverter,
    TypeConverter<T, Row> ObjectConverter,
    IMapper<T, T> RowsMapper,
    IMapper<string, string> ColumnMappings,
    IMapper<string, string> EntityMapper,
    RepositoryStatements Statements,
    IReadOnlyList<string> Ids) where T : new();
