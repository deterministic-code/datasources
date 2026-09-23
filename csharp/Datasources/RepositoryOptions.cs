namespace Datasources;

public sealed class RepositoryOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public string? Entity { get; init; }
    public TypeConverter<T, Row>? Mapping { get; init; }
    public IMapper<T, T>? Mapper { get; init; }
    public CRUDMapper<T>? CRUDMapper { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? Fields { get; init; }
    public IReadOnlyList<string>? Columns { get; init; }
}
