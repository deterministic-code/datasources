using Datasources;
using Datasources.Text;

public partial class RegisterCreateTests
{
    const string Provider = "text";
    static readonly IDatasourceFactory Factory = TextDatasource.Factory;
}
