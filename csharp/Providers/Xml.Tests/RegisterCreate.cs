using Datasources;
using Datasources.Xml;

public partial class RegisterCreateTests
{
    const string Provider = "xml";
    static readonly IDatasourceFactory Factory = XmlDatasource.Factory;
}
