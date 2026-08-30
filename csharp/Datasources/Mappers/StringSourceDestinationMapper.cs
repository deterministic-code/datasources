namespace Datasources;

public sealed class StringSourceDestinationMapper(string source, string destination)
    : StringMapper, ISourceDestinationMapper
{
    public string Source { get; } = source;
    public string Destination { get; } = destination;
}
