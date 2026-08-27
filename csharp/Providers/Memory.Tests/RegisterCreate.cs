using Datasources;
using Datasources.Memory;

public partial class RegisterCreateTests
{
    const string Provider = "memory";
    static readonly IDatasourceFactory Factory = MemoryDatasource.Factory;
}
