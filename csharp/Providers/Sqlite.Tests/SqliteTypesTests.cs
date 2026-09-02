using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteTypesTests
{
    [Fact]
    public void RegistersSqliteTypes()
    {
        _ = SqliteTypes.All;
        var types = DefaultTypeConverters.For("sqlite");
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
