namespace Datasources;

public sealed record BuiltRepository<T>(
    IConnection Connection,
    TypeConverter<Row, Row>? RowConverter,
    TypeConverter<T, Row> ObjectConverter,
    IMapper<string, string> PropertyMappings,
    IMapper<string, string> EntityMapper,
    ReadonlyRepositoryStatements Statements,
    IReadOnlyList<string> Ids) where T : new();
