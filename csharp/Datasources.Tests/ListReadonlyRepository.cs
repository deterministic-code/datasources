using Datasources;

namespace Datasources.Tests;

internal sealed class ListReadonlyRepository<T> : ReadonlyRepository<T> where T : new()
{
    readonly IReadOnlyList<T> items;
    public ListReadonlyRepository(params T[] items) => this.items = items;

    public override async IAsyncEnumerable<T> Read(IStatement? query = null)
    {
        await Task.Yield();
        foreach (var item in items)
            if (Matches(item, query))
                yield return item;
    }

    protected override IAsyncEnumerable<(T Item, Row? Source)> ReadTracked(IStatement? query = null) =>
        WithoutSource(Read(query));

    static bool Matches(T item, IStatement? query) =>
        query is IQuery { Where: { } where } && where.GetText() == "1 = 0"
            ? false
            : ParametersOf(query)
                .GroupBy(parameter => FieldOf(parameter.Name))
                .All(group => group.Any(parameter => Same(Get(item, group.Key), parameter.Value)));

    static string FieldOf(string name)
    {
        var split = name.LastIndexOf('_');
        return split > 0 && name[(split + 1)..].All(char.IsDigit) ? name[..split] : name;
    }

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
