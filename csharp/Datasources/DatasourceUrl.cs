namespace Datasources;

public sealed record DatasourceUrl(
    string Provider,
    string? Username,
    string? Password,
    string? Host,
    int? Port,
    string Path,
    string? Schema,
    IReadOnlyDictionary<string, string> Options)
{
    public string? Datasource => Path switch
    {
        "" => null,
        ['/', .. var rest] => rest.Length == 0 ? null : rest,
        _ => Path
    };
}
