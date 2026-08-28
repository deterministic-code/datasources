using Datasources;

namespace Datasources.Json;

public static class JsonConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("json", Types);
}
