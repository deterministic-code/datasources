using Datasources;

namespace Datasources.Yaml;

internal sealed class YamlCommand : StubCommand
{
    public YamlCommand(IConnection connection) : base("yaml", connection) { }
}
