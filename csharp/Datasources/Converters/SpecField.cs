namespace Datasources;

public sealed class SpecField
{
    public string Type { get; init; } = "";
    public string? Name { get; init; }
    public object? Size { get; init; }
    public object? DefaultValue { get; init; }
}
