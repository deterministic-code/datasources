using Datasources;
using Datasources.DynamoDb;

public partial class RegisterCreateTests
{
    const string Provider = "dynamodb";
    static readonly IDatasourceFactory Factory = DynamoDbDatasource.Factory;
}
