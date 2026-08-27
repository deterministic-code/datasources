using Datasources;
using Datasources.Yaml;

public partial class RegisterCreateTests
{
    const string Provider = "yaml";
    static readonly IDatasourceFactory Factory = YamlDatasource.Factory;
}
