namespace Datasources;

public sealed class CRUDRepositoryOptions<T> : ReadonlyRepositoryOptions<T> where T : new()
{
    public IReadOnlyList<string> Ids { get; init; } = ["id"];
    public IMapper<IInsertStatement, IInsertStatement>? Insert { get; init; }
    public IMapper<IUpdateStatement, IUpdateStatement>? Update { get; init; }
    public IMapper<IDeleteStatement, IDeleteStatement>? Delete { get; init; }
    public IMapper<IReturnStatement, IReturnStatement>? Return { get; init; }
    public IMapper<Value, T>? Create { get; init; }

    internal static IMapper<Value, T> Identity(IReadOnlyList<string> ids) =>
        new TransformMapper<Value, T>(
            value =>
            {
                var item = new T();
                if (value is Value.Null || ids.Count != 1) return item;
                var name = ids[0];
                if (item is System.Collections.IDictionary dict)
                    dict[name] = item is Row ? value : value.ToObject();
                else if (typeof(T).GetProperty(name) is { } prop)
                    prop.SetValue(item, Coerce(value.ToObject(), prop.PropertyType));
                return item;
            },
            static _ => Value.Null.Instance);

    static object? Coerce(object? value, Type type)
    {
        type = Nullable.GetUnderlyingType(type) ?? type;
        return value is null || type.IsInstanceOfType(value) ? value : Convert.ChangeType(value, type);
    }
}
