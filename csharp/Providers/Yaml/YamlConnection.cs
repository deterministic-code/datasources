using Datasources;

namespace Datasources.Yaml;

public sealed class YamlConnection : StubConnection
{
    public YamlConnection() : base("yaml") { }
}
