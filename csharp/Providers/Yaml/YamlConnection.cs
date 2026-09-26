using Datasources;

namespace Datasources.Yaml;

internal sealed class YamlConnection : StubConnection
{
    public YamlConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("yaml") { }
}
