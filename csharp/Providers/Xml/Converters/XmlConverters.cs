using Datasources;

namespace Datasources.Xml;

public static class XmlConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("xml", Types);
}
