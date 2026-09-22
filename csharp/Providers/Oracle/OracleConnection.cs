using Datasources;

namespace Datasources.Oracle;

public sealed class OracleConnection : StubConnection
{
    public OracleConnection(IReadOnlyDictionary<string, string?>? _ = null) : base("oracle") { }
    public override ICommand CreateCommand() => new OracleCommand(this);

}
