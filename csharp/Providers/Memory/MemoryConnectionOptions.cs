namespace Datasources.Memory;

internal sealed class MemoryConnectionOptions
{
    public string? Collection { get; set; }
    public MemoryEngine? Engine { get; set; }
    public object? Data { get; set; }
}
