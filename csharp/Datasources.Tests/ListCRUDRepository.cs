namespace Datasources.Tests;

internal sealed class ListCRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    public List<T> Items { get; } = [];
    int nextId = 1;

    public ListCRUDRepository(params T[] items)
    {
        Items.AddRange(items);
        nextId = items.Select(IdInt).DefaultIfEmpty(0).Max() + 1;
    }

    public override async IAsyncEnumerable<T> Read(IStatement? query = null)
    {
        await Task.Yield();
        foreach (var item in Items)
            if (Matches(item, query))
                yield return item;
    }

    protected internal override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        WithoutSource(Read(query));

    public Task<Response> Create(T item)
    {
        if (Id(item) is not { } id || !HasId(id))
            SetId(item!, nextId++);
        Items.Add(item);
        return Task.FromResult(new Response(Value.From(Id(item)), 1));
    }

    public Task<Response> Update(T item, IWhere? byWhere = null)
    {
        var key = IdKey(Id(item));
        var index = Items.FindIndex(row => IdKey(Id(row)) == key);
        if (index >= 0)
            Items[index] = item;
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Delete(T item, IWhere? byWhere = null)
    {
        var key = IdKey(Id(item));
        Items.RemoveAll(row => IdKey(Id(row)) == key);
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    static bool Matches(T item, IStatement? query) =>
        ParametersOf(query).All(parameter => Same(Get(item, parameter.Name), parameter.Value));

    static IReadOnlyList<Parameter> ParametersOf(IStatement? query) =>
        query switch
        {
            IQuery q => q.Where?.Parameters ?? q.Parameters,
            IWhere w => w.Parameters,
            null => [],
            _ => query.Parameters
        };

    static object? Get(T item, string name) =>
        typeof(T).GetProperty(name)?.GetValue(item);

    static bool Same(object? left, Value right)
    {
        var boxed = right.ToObject();
        return left is null && boxed is null
            || left is not null && boxed is not null
                && (Equals(left, boxed) || string.Equals(Convert.ToString(left), Convert.ToString(boxed)));
    }

    static object? Id(T item)
    {
        var type = typeof(T);
        return (type.GetProperty("id") ?? type.GetProperty("Id"))?.GetValue(item);
    }

    static int IdInt(T item) =>
        Id(item) is IConvertible convertible ? convertible.ToInt32(null) : 0;

    static bool HasId(object id) =>
        id is not 0 and not 0L and not 0u and not "";

    static string IdKey(object? id) => id?.ToString() ?? "";

    static void SetId(object item, int id)
    {
        var prop = item.GetType().GetProperty("id") ?? item.GetType().GetProperty("Id");
        if (prop is null) return;
        var target = Nullable.GetUnderlyingType(prop.PropertyType) ?? prop.PropertyType;
        prop.SetValue(item, Convert.ChangeType(id, target));
    }
}
