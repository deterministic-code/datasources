using Datasources;

namespace Datasources.MongoDb;

public static class MongoDbConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("mongodb", Types);
}
