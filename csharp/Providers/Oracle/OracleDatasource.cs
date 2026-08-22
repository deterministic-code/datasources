using Datasources;

namespace Datasources.Oracle;

public static class OracleProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("oracle");

    static OracleProvider()
    {
        Datasource.Use(Factory);
    }
}
