using Datasources;
using Datasources.Json;

public partial class RegisterCreateTests
{
    const string Provider = "json";
    static readonly IDriver Driver = new JsonProvider();
}
