using Datasources;

namespace Datasources.Tests;

internal sealed class ListReadonlyRepository<T> : ReadonlyRepository<T>
{
    readonly IReadOnlyList<T> items;
    public ListReadonlyRepository(params T[] items) => this.items = items;

    public override async IAsyncEnumerable<T> Get(IStatement? query = null)
    {
        await Task.Yield();
        foreach (var item in items)
            if (Matches(item, query))
                yield return item;
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
}
