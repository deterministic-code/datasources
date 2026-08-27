using Datasources;

namespace Datasources.Json;

public sealed class MemoryFileStore : IFileStore
{
    public Dictionary<string, string> Files { get; } = new();
    public Task<string> Read(string path) =>
        Files.TryGetValue(path, out var contents)
            ? Task.FromResult(contents)
            : throw new DatasourceException($"File not found: {path}");
    public Task Write(string path, string contents)
    {
        Files[path] = contents;
        return Task.CompletedTask;
    }
    public Task<bool> Exists(string path) => Task.FromResult(Files.ContainsKey(path));
}
