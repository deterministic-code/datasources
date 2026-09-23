namespace Datasources;

public sealed class DictionaryMapper<T>(IReadOnlyDictionary<T, T> map) : IMapper<T, T> where T : notnull
{
    readonly Dictionary<T, T> forward = new(map);
    readonly Dictionary<T, T> reverse = Reverse(map);

    public T Map(T value) => forward.TryGetValue(value, out var to) ? to : value;

    public T ReverseMap(T value) => reverse.TryGetValue(value, out var from) ? from : value;

    static Dictionary<T, T> Reverse(IReadOnlyDictionary<T, T> map)
    {
        var reverse = new Dictionary<T, T>();
        foreach (var (from, to) in map)
            reverse.TryAdd(to, from);
        return reverse;
    }
}
