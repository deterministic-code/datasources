using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class UuidConverterTests
{
    [Fact]
    public void Uuid()
    {
        var converter = new UuidConverter();
        var id = System.Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        var native = converter.Convert("550e8400-e29b-41d4-a716-446655440000");
        Assert.Equal(new Value.Guid(id), native);
        Assert.Equal(native, converter.ReverseConvert(native));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Int(1), converter.Convert(1L));
        Assert.Equal(new Value.Int(1), converter.ReverseConvert(1L));
    }
}
