using System.Text.Json.Nodes;

namespace Datasources.Json;

internal sealed class JsonConnectionOptions
{
    public string Path { get; set; } = "";
    public string? Collection { get; set; }
    public IFileStore? FileStore { get; set; }
    public JsonNode? Data { get; set; }
}
