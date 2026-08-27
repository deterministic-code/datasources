using System.Text.Json.Nodes;

namespace Datasources.Json;

public sealed class JsonConnectionOptions
{
    public string Path { get; set; } = "";
    public string? Collection { get; set; }
    public IFileStore? FileStore { get; set; }
    public JsonNode? Data { get; set; }
}
