using Datasources;
using Datasources.Mysql;

public partial class RegisterCreateTests
{
    const string Provider = "mysql";
    static readonly IDriver Driver = new MysqlProvider();
}

public class PublicSurfaceTests
{
    [Fact]
    public void ExportsOnlyTheProvider() =>
        Assert.Equal(typeof(MysqlProvider), Assert.Single(typeof(MysqlProvider).Assembly.GetExportedTypes()));
}
