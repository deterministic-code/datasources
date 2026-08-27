using Datasources;

namespace Datasources.Oracle;

public sealed class OracleCommand : StubCommand
{
    public OracleCommand(IConnection connection) : base("oracle", connection) { }
}
