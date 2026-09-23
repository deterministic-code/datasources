using Datasources;
using static Datasources.Cmp;

namespace Datasources.Tests;

public class ComparisonAliasTests
{
    [Fact]
    public void RendersComparisonSymbols()
    {
        var equals = new WhereClause("username", eq, "grace");
        var notEquals = new WhereClause("id", ne, 2);
        var greater = new WhereClause("n", gt, 0);
        var less = new WhereClause("n", lt, 0);
        var greaterOrEqual = new WhereClause("n", ge, 0);
        var lessOrEqual = new WhereClause("n", le, 0);
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
