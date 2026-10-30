using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => OracleDriver.Quote(name);
}
