using Datasources;

namespace Datasources.Xml;

public static class XmlConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("xml", Types);
}
