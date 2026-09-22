using Datasources;
using Datasources.Mysql;

public partial class RegisterCreateTests
{
    const string Provider = "mysql";
    static readonly IDriver Driver = new MysqlProvider();
}
