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
        Assert.Equal("username = $username", DefaultStatementTextRenderer.Instance.Render(equals));
        Assert.Equal("id <> $id", DefaultStatementTextRenderer.Instance.Render(notEquals));
        Assert.Equal("n > $n", DefaultStatementTextRenderer.Instance.Render(greater));
        Assert.Equal("n < $n", DefaultStatementTextRenderer.Instance.Render(less));
        Assert.Equal("n >= $n", DefaultStatementTextRenderer.Instance.Render(greaterOrEqual));
        Assert.Equal("n <= $n", DefaultStatementTextRenderer.Instance.Render(lessOrEqual));
        Assert.Equal("username = $username AND is_enabled = $is_enabled", DefaultStatementTextRenderer.Instance.Render(both));
        Assert.Equal("username = $username OR is_enabled = $is_enabled", DefaultStatementTextRenderer.Instance.Render(either));
    }
}
