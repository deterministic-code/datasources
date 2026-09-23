using Datasources;
using Datasources.Sqlite;

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

    [Fact]
    public void MicrosoftAssemblyExportsNothing() =>
        Assert.Empty(typeof(SqliteConnectionDriver).Assembly.GetExportedTypes());
}
