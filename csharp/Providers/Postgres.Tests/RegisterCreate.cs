using Datasources;
using Datasources.Postgres;

public partial class RegisterCreateTests
{
    const string Provider = "postgres";
    static readonly IDriver Driver = new PostgresDriver();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(PostgresDriver), Assert.Single(typeof(PostgresDriver).Assembly.GetExportedTypes()));
}
