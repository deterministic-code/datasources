using Datasources;

namespace Datasources.Text;

public static class TextConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("text", Types);
}
