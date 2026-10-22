using Datasources;
using Datasources.Sqlite;

[Collection("DatasourceFactory")]
public partial class RegisterCreateTests
{
    const string Provider = "sqlite";
    static readonly IDriver Driver = new SqliteDriver();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(SqliteDriver), Assert.Single(typeof(SqliteDriver).Assembly.GetExportedTypes()));

}
