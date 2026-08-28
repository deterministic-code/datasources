using Datasources;
using Datasources.Postgres;

public partial class RegisterCreateTests
{
    const string Provider = "postgres";
    static readonly IDatasourceFactory Factory = PostgresDatasource.Factory;
}
