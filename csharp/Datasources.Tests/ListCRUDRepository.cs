namespace Datasources.Tests;

internal sealed class ListCRUDRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    public List<T> Items { get; } = [];
    public IReadOnlyList<string> Ids { get; } = CreateRepositoryOptions.FromType<T>().Ids;
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

    protected override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        WithoutSource(Read(query));

    public Task<T> Create(T item)
    {
        if (Id(item) is not { } id || !HasId(id))
            SetId(item!, nextId++);
        Items.Add(item);
        return Task.FromResult(item);
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
        if (byWhere is not null)
            Items.RemoveAll(row => byWhere.Parameters.All(parameter => Same(Get(row, parameter.Name), parameter.Value)));
        else
            Items.RemoveAll(row => IdKey(Id(row)) == IdKey(Id(item)));
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<T?> ReadSingleById(Value id) => Task.FromResult(Find(item => Same(Id(item), id)));

    public Task<T?> ReadSingleById(Row ids) =>
        Task.FromResult(Find(item => ids.All(pair => Same(Get(item, pair.Key), pair.Value))));

    T? Find(Func<T, bool> match)
    {
        var index = Items.FindIndex(item => match(item));
        return index < 0 ? default : Items[index];
    }

    static bool Matches(T item, IStatement? query) =>
        ParametersOf(query).All(parameter => Same(Get(item, parameter.Name), parameter.Value));

    static IReadOnlyList<Parameter> ParametersOf(IStatement? query) =>
        query switch
        {
            ISelectStatement q => q.Where?.Parameters ?? q.Parameters,
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
