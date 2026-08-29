namespace Datasources;

public sealed class TypeMapping<T> where T : new()
{
    public required string Name { get; init; }
    public required string Source { get; init; }
    public required string Destination { get; init; }
    public required IReadOnlyDictionary<string, string> Schema { get; init; }
    public required Func<IReadOnlyDictionary<string, Value>, T> To { get; init; }
    public required Func<T, IReadOnlyDictionary<string, Value>> From { get; init; }
}
