using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlTypesTests
{
    [Fact]
    public void RegistersMysqlTypes()
    {
        var types = new MysqlProvider().Types;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
