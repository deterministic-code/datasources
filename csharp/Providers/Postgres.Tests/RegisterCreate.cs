using Datasources;
using Datasources.Postgres;

public partial class RegisterCreateTests
{
    const string Provider = "postgres";
    static readonly IDriver Driver = new PostgresProvider();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(PostgresProvider), Assert.Single(typeof(PostgresProvider).Assembly.GetExportedTypes()));
}
