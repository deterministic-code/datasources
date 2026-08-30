using Datasources;

namespace Datasources.DynamoDb;

public static class DynamoDbConverters
{
    public static ProviderTypes Types { get; } = new() { Rename = false };

    public static void Register() => TypeConverter.Use("dynamodb", Types);
}
