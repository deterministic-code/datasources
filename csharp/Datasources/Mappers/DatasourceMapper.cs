namespace Datasources;

public sealed class DatasourceMapper
{
    public string Source { get; }
    public string Destination { get; }

    readonly Dictionary<string, string> to;
    readonly Dictionary<string, string> from;

    DatasourceMapper(string source, string destination, IReadOnlyDictionary<string, string> names)
    {
        Source = source;
        Destination = destination;
        to = new(names);
        from = names.ToDictionary(pair => pair.Value, pair => pair.Key);
    }

    public static DatasourceMapper Mapping(
        string source,
        string destination,
        IReadOnlyDictionary<string, string>? names = null) =>
        new(source, destination, names ?? new Dictionary<string, string>());

    public string To(string name) => to.GetValueOrDefault(name, name);
    public string From(string name) => from.GetValueOrDefault(name, name);
}
