using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleCommand : StubCommand
{
    public OracleCommand(IConnection connection) : base("oracle", connection) { }
}
