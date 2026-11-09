namespace Datasources.Tests;

public class IdentityTests
{
    [Fact]
    public void SchemaIdsUsesIsId()
    {
        var schema = new Schema(new Dictionary<string, SchemaColumn>
        {
            ["Key"] = new("string", IsId: true),
            ["Name"] = new("string"),
        });
        Assert.Equal(["Key"], Identity.Ids(schema));
    }
}
