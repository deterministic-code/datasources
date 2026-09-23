using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteRowTypeConvertersTests
{
    [Fact]
    public void RegistersSqliteRowTypeConverters()
    {
        var types = new SqliteDriver().RowTypeConverters;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
