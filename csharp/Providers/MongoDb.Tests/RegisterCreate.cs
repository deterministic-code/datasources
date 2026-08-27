using Datasources;
using Datasources.MongoDb;

public partial class RegisterCreateTests
{
    const string Provider = "mongodb";
    static readonly IDatasourceFactory Factory = MongoDbDatasource.Factory;
}
