namespace Datasources;

public sealed class NameSourceDestinationMapper(string source, string destination)
    : NameMapper, ISourceDestinationMapper
{
    public string Source { get; } = source;
    public string Destination { get; } = destination;
}
