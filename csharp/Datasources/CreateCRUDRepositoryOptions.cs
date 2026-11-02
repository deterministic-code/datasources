namespace Datasources;

public static class CreateCRUDRepositoryOptions
{
    public static CRUDRepositoryOptions<T> FromType<T>(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), types);

    public static CRUDRepositoryOptions<T> FromType<T>(
        Type type,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new()
    {
        var read = CreateReadonlyRepositoryMappingOptions.FromType<T>(type, types);
        return new()
        {
            Ids = read.Schema?.Ids is { Count: > 0 } ids ? ids : NamedId(type),
            Entity = read.Entity,
            Schema = read.Schema,
            RowTypeConverters = read.RowTypeConverters
        };
    }

    static IReadOnlyList<string> NamedId(Type type)
    {
        if (type.GetProperty("id") is not null) return ["id"];
        if (type.GetProperty("Id") is not null) return ["Id"];
        return [];
    }
}
