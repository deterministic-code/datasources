using Datasources;

namespace Datasources.SqlServer;

public static class SqlServerProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("sqlserver");

    static SqlServerProvider()
    {
        Datasource.Use(Factory);
    }
}
