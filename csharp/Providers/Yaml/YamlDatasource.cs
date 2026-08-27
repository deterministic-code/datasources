using Datasources;

namespace Datasources.Yaml;

public sealed class YamlDatasource : IDatasourceFactory
{
    public static readonly YamlDatasource Factory = new();
    public string Provider => "yaml";
    public IDatasource Create(object? options = null) => new StubDatasource("yaml");
}
