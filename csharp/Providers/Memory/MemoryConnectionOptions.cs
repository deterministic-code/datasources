namespace Datasources.Memory;

internal sealed class MemoryConnectionOptions
{
    public string? Collection { get; set; }
    public MemoryStore? Store { get; set; }
    public object? Data { get; set; }
}
