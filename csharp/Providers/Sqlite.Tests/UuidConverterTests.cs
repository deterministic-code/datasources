using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class UuidConverterTests
{
    [Fact]
    public void Uuid()
    {
        var converter = new UuidConverter();
        var id = System.Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        var text = new Value.Text("550e8400-e29b-41d4-a716-446655440000");
        Assert.Equal(text, converter.Convert("550e8400-e29b-41d4-a716-446655440000"));
        Assert.Equal(text, converter.Convert(id));
        Assert.Equal(text, converter.Convert(id.ToByteArray()));
        Assert.Equal(text, converter.ReverseConvert(id));
        Assert.Equal(new Value.Text("nope"), converter.Convert("nope"));
        Assert.Equal(new Value.Int(1), converter.Convert(1L));
        Assert.Equal(new Value.Bool(true), converter.Convert(true));
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(true));
    }
}
