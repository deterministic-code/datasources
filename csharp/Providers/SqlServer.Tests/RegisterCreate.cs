using Datasources;
using Datasources.SqlServer;

public partial class RegisterCreateTests
{
    const string Provider = "sqlserver";
    static readonly IDatasourceFactory Factory = SqlServerDatasource.Factory;
}
