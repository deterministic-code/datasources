using Datasources;
using Datasources.Oracle;

public partial class RegisterCreateTests
{
    const string Provider = "oracle";
    static readonly IDatasourceFactory Factory = OracleDatasource.Factory;
}
