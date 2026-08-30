using Datasources;

namespace Datasources.Json;

public static class JsonConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("json", Types);
}
