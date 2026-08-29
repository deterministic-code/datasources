using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static readonly IDatasourceFactory Instance = JsonDatasource.Instance;
}
