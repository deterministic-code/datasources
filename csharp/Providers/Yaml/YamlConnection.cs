using Datasources;

namespace Datasources.Yaml;

public sealed class YamlConnection : StubConnection
{
    public YamlConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("yaml") { }
    public override ICommand CreateCommand() => new YamlCommand(this);
    public static object Repository(IConnection connection) =>
        new StubRepository<object>(connection.Provider, connection);
}
