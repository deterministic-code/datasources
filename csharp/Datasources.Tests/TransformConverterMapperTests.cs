using Datasources;

namespace Datasources.Tests;

public class TransformConverterMapperTests
{
    sealed class Wrap : TypeConverter<string, string>
    {
        public override string Convert(string from) => $"[{from}]";
        public override string ReverseConvert(string to) => to.Trim('[', ']');
    }

    [Fact]
    public void ConvertsBothWaysWithoutFilter()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap());
        Assert.Equal("[ab]", mapper.Map("ab"));
        Assert.Equal("ab", mapper.ReverseMap("[ab]"));
    }

    [Fact]
    public void ConvertsWhenFilterIsTrue()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap(), s => s.Contains("ab"));
        Assert.Equal("[ab]", mapper.Map("ab"));
        Assert.Equal("ab", mapper.ReverseMap("[ab]"));
    }

    [Fact]
    public void IsIdentityWhenFilterIsFalse()
    {
        var mapper = new TransformConverterMapper<string>(new Wrap(), s => s.Contains("ab"));
        Assert.Equal("a", mapper.Map("a"));
        Assert.Equal("[a]", mapper.ReverseMap("[a]"));
    }
}
