using Datasources;

namespace Datasources.Tests;

public class EmptyDefaultValueConverterTests
{
    [Fact]
    public void ConvertReturnsNull() =>
        Assert.Null(EmptyDefaultValueConverter.Instance.Convert("string", "hi"));

    [Fact]
    public void ReverseConvertThrows() =>
        Assert.Equal(
            "Native default 'hi' is not a valid string default",
            Assert.Throws<InvalidOperationException>(() =>
                EmptyDefaultValueConverter.Instance.ReverseConvert("string", "hi")).Message);
}
