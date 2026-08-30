using Datasources;

namespace Datasources.Yaml;

public static class YamlConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("yaml", Types);
}
