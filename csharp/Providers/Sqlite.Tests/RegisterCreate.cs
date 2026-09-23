using Datasources;
using Datasources.Sqlite;

public partial class RegisterCreateTests
{
    const string Provider = "sqlite";
    static readonly IDriver Driver = new SqliteProvider();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(SqliteProvider), Assert.Single(typeof(SqliteProvider).Assembly.GetExportedTypes()));

    [Fact]
    public void MicrosoftAssemblyExportsNothing() =>
        Assert.Empty(typeof(SqliteDriver).Assembly.GetExportedTypes());
}
