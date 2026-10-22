using Datasources.CodeGenerators;

namespace Datasources.CodeGenerators.Tests;

public class GenerateSqlTests
{
    [Fact]
    public void ThrowsWhenTheDialectIsUnknown() =>
        Assert.Throws<InvalidOperationException>(() =>
            GenerateSql.Of(new Dictionary<string, string> { ["types.yaml"] = "types: []" }, new Dictionary<string, string>(), ["acme"]));
}
