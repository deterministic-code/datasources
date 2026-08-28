using Datasources;

namespace Datasources.Yaml;

public static class YamlConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("yaml", Types);
}
