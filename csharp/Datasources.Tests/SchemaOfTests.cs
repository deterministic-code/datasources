namespace Datasources.Tests;

public class SchemaOfTests
{
    public class User
    {
        public long Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
        public DateTime Created { get; set; }
        public Guid? Key { get; set; }
    }

    [Fact]
    public void FromMapsReflectionToCanonicalTypes()
    {
        var schema = SchemaOf.From<User>();
        Assert.Equal("biginteger", schema.Types["Id"]);
        Assert.False(schema.Types.ContainsKey("Username"));
        Assert.Equal("boolean", schema.Types["IsEnabled"]);
        Assert.Equal("datetime", schema.Types["Created"]);
        Assert.Equal("uuid", schema.Types["Key"]);
        Assert.True(schema.Columns["Key"].Nullable);
    }

    [Fact]
    public void FromDictionaryIsEmpty() =>
        Assert.Empty(SchemaOf.From<Dictionary<string, object>>().Columns);
}
