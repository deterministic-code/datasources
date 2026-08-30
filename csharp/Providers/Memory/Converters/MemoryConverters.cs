using Datasources;

namespace Datasources.Memory;

public static class MemoryConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("memory", Types);
}
