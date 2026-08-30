namespace Datasources;

public sealed class Mapper(string name, string source, string destination)
{
    public string Name { get; } = name;
    public string Source { get; } = source;
    public string Destination { get; } = destination;

    public string To(string name) => name;
    public string From(string name) => name;
}
