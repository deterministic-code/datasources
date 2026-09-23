using Datasources;
using Datasources.Mysql;

public partial class RegisterCreateTests
{
    const string Provider = "mysql";
    static readonly IDriver Driver = new MysqlDriver();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(MysqlDriver), Assert.Single(typeof(MysqlDriver).Assembly.GetExportedTypes()));
}
