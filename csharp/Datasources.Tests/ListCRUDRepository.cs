namespace Datasources.Tests;

internal sealed class ListCRUDRepository<T> : BulkCRUDRepository<T> where T : new()
{
    public List<T> Items { get; } = [];
    int nextId = 1;

    public ListCRUDRepository(params T[] items) : this(null, items) { }

    public ListCRUDRepository(IWriteEnrichmentStrategy<T>? write, params T[] items) : base(write)
    {
        Items.AddRange(items);
        nextId = items.Select(IdInt).DefaultIfEmpty(0).Max() + 1;
    }

    protected override async IAsyncEnumerable<(T Item, Row? Source)> ReadItems(
        IStatement? query = null, IReadOnlyList<Parameter>? parameters = null)
    {
        await Task.Yield();
        foreach (var item in Items)
            if ((parameters ?? []).All(parameter => Same(Get(item, parameter.Name), parameter.Value)))
                yield return (item, null);
    }

    protected override Task<T> Inserted(T item)
    {
        if (Id(item) is not { } id || !HasId(id))
            SetId(item!, nextId++);
        Items.Add(item);
        return Task.FromResult(item);
    }

    protected override Task<Response> Save(T item, IWhere? byWhere)
    {
        var key = IdKey(Id(item));
        var index = Items.FindIndex(row => IdKey(Id(row)) == key);
        if (index >= 0)
            Items[index] = item;
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    protected override Task<Response> Remove(T item, IWhere? byWhere)
    {
        if (byWhere is not null)
            Items.RemoveAll(row => byWhere.Parameters.All(parameter => Same(Get(row, parameter.Name), parameter.Value)));
        else
            Items.RemoveAll(row => IdKey(Id(row)) == IdKey(Id(item)));
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

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
