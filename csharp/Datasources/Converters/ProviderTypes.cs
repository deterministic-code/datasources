namespace Datasources;

public sealed class ProviderTypes
{
    public bool Rename { get; init; } = true;
    public Dictionary<string, IFieldConverter> Types { get; init; } = [];
}
