using Datasources;

namespace Datasources.DynamoDb;

public static class DynamoDbConverters
{
    public static ProviderTypes Types { get; } = TypeConverter.IdentityProviderTypes();

    public static void Register() => TypeConverter.Use("dynamodb", Types);
}
