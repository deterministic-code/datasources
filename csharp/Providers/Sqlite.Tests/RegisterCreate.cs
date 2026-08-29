using Datasources;
using Datasources.Sqlite;

public partial class RegisterCreateTests
{
    const string Provider = "sqlite";
    static readonly IDatasourceFactory Instance = SqliteDatasource.Instance;
}
