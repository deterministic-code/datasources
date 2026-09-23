namespace Datasources.Json;

internal interface IFileStore
{
    Task<string> Read(string path);
    Task Write(string path, string contents);
    Task<bool> Exists(string path);
}
