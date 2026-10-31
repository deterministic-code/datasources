using Datasources;

namespace Datasources.Tests;

public class SchemaDefaultTests
{
    [Fact]
    public void NullableIntegerIsNull() =>
        Assert.Equal(Value.Null.Instance, SchemaDefault.Of(new SchemaColumn("integer", Nullable: true)));

    [Fact]
    public void RequiredIntegerIsZero() =>
        Assert.Equal(0L, SchemaDefault.Of(new SchemaColumn("integer")));

    [Fact]
    public void RequiredStringIsEmpty() =>
        Assert.Equal("", SchemaDefault.Of(new SchemaColumn("string")));

    [Fact]
    public void RequiredBooleanIsFalse() =>
        Assert.Equal(false, SchemaDefault.Of(new SchemaColumn("boolean")));
}
