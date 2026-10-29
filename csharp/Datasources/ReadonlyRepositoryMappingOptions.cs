namespace Datasources;

public record ReadonlyRepositoryMappingOptions<T>
{
    public string? Entity { get; init; }
    public IMapper<string, string>? EntityMapper { get; init; }
    public IMapper<string, string>? PropertyMappings { get; init; }
    public TypeConverter<Row, Row>? RowConverter { get; init; }
    public TypeConverter<T, Row>? ObjectConverter { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
    public IReadOnlyList<string>? ReadProperties { get; init; }
    public Schema? Schema { get; init; }
}
