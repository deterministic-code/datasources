namespace Datasources;

public sealed class NameSourceDestinationMapper(string source, string destination)
    : INameMapper
{
    public string Source { get; } = source;
    public string Destination { get; } = destination;
    public string Map(string name) => name;
    public string ReverseMap(string name) => name;
}
