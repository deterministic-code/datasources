using Datasources;

namespace Datasources.Yaml;

public sealed class YamlCommand : StubCommand
{
    public YamlCommand(IConnection connection) : base("yaml", connection) { }
}
