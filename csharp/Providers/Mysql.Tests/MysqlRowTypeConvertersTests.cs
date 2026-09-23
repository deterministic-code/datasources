using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlRowTypeConvertersTests
{
    [Fact]
    public void RegistersMysqlRowTypeConverters()
    {
        var types = new MysqlDriver().RowTypeConverters;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
