namespace Datasources;

public sealed class RepositoryOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public string? Entity { get; init; }
    public TypeConverter<T, Row>? RowConverter { get; init; }
    public CRUDMapperOptions<T>? CRUDMapper { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? RowTypeConverters { get; init; }
    public IReadOnlyList<string>? ReadColumns { get; init; }
}
