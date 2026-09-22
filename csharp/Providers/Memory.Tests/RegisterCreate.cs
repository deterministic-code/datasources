using Datasources;
using Datasources.Memory;

public partial class RegisterCreateTests
{
    const string Provider = "memory";
    static readonly IDriver Driver = new MemoryProvider();
}
