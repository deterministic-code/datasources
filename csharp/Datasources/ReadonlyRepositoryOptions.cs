namespace Datasources;

public class ReadonlyRepositoryOptions<T> where T : new()
{
    public string? Entity { get; init; }
    public IMapper<string, string> EntityMapper { get; init; } = new IdentityMapper<string>();
    public IMapper<string, string> ColumnMappings { get; init; } = new IdentityMapper<string>();
    public IMapper<ISelectStatement, ISelectStatement>? Query { get; init; }
    public IMapper<T, T> RowsMapper { get; init; } = new IdentityMapper<T>();
    public TypeConverter<T, Row>? RowConverter { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
    public IReadOnlyList<string>? ReadProperties { get; init; }
}
