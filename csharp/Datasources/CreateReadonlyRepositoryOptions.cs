namespace Datasources;

public static class CreateReadonlyRepositoryOptions
{
    public static ReadonlyRepositoryOptions<T> FromType<T>(
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        FromType<T>(typeof(T), types);

    public static ReadonlyRepositoryOptions<T> FromType<T>(
        Type type,
        IReadOnlyDictionary<string, TypeConverter<Value, Value>>? types = null)
        where T : new() =>
        new()
        {
            Entity = type.Name,
            RowTypeConverters = types
        };
}
