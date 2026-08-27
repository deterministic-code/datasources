using Datasources;

namespace Datasources.Oracle;

public sealed class OracleDatasource : IDatasourceFactory
{
    public static readonly OracleDatasource Factory = new();
    public string Provider => "oracle";
    public IDatasource Create(object? options = null) => new StubDatasource("oracle");
}
