using Datasources;

namespace Datasources.MongoDb;

public static class MongoDbConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("mongodb", Types);
}
