using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlTypesTests
{
    [Fact]
    public void RegistersMysqlTypes()
    {
        _ = MysqlTypes.All;
        var types = DefaultTypeConverters.For("mysql");
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
