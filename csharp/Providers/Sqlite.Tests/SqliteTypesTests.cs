using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteTypesTests
{
    [Fact]
    public void RegistersSqliteTypes()
    {
        var types = new SqliteProvider().Types;
        Assert.Equal(["bool", "datetime", "uuid", "decimal", "binary"], types.Keys);
    }
}
