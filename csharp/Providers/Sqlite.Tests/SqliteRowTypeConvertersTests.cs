using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteRowTypeConvertersTests
{
    [Fact]
    public void RegistersSqliteRowTypeConverters()
    {
        var types = new SqliteDriver().RowTypeConverters;
        Assert.Equal(["boolean", "datetime", "uuid", "decimal", "binary", "integer"], types.Keys);
    }
}
