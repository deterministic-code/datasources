namespace Datasources;

public sealed class EnrichmentSelection
{
    readonly HashSet<string> names;

    EnrichmentSelection(IEnumerable<string> names) => this.names = new HashSet<string>(names);

    public static EnrichmentSelection None { get; } = new([]);

    public static EnrichmentSelection Only(params string[] names) => new(names);

    public bool Includes(string destination) => names.Contains(destination);
}
