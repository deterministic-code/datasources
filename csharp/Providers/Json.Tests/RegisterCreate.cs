using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static readonly IDatasourceFactory Factory = JsonDatasource.Factory;
}
