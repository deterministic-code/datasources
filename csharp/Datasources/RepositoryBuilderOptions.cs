namespace Datasources;

public sealed class RepositoryBuilderOptions<T> where T : new()
{
    public required IConnection Connection { get; init; }
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public bool AssignIdentityOnAdd { get; init; } = true;
    public string? Entity { get; init; }
    public TypeConverter<T, Row>? Mapping { get; init; }
    public IMapper<T, T>? Mapper { get; init; }
    public IReadOnlyDictionary<Type, TypeConverter<object?, Value>>? Converters { get; init; }
    public IReadOnlyDictionary<string, TypeConverter<Value, Value>>? Fields { get; init; }
    public IReadOnlyList<string>? Columns { get; init; }
}
