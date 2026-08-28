using Datasources;

namespace Datasources.Memory;

public static class MemoryConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("memory", Types);
}
