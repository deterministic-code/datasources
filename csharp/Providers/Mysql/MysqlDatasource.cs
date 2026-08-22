using Datasources;

namespace Datasources.Mysql;

public static class MysqlProvider
{
    public static readonly StubDatasource Factory = StubFactory.Create("mysql");

    static MysqlProvider()
    {
        Datasource.Use(Factory);
    }
}
