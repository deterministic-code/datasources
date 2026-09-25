using Datasources;
using static Datasources.Cmp;

namespace Datasources.Tests;

public class ComparisonAliasTests
{
    [Fact]
    public void RendersComparisonSymbols()
    {
        var equals = new Where("username", eq, "grace");
        var notEquals = new Where("id", ne, 2);
        var greater = new Where("n", gt, 0);
        var less = new Where("n", lt, 0);
        var greaterOrEqual = new Where("n", ge, 0);
        var lessOrEqual = new Where("n", le, 0);
        var both = equals.And("is_enabled", eq, 0);
        var either = equals.Or("is_enabled", eq, 0);
        Assert.Equal("username = $username", equals.GetText());
        Assert.Equal("id <> $id", notEquals.GetText());
        Assert.Equal("n > $n", greater.GetText());
        Assert.Equal("n < $n", less.GetText());
        Assert.Equal("n >= $n", greaterOrEqual.GetText());
        Assert.Equal("n <= $n", lessOrEqual.GetText());
        Assert.Equal("username = $username AND is_enabled = $is_enabled", both.GetText());
        Assert.Equal("username = $username OR is_enabled = $is_enabled", either.GetText());
    }
}
