using Datasources;
using Datasources.CodeGenerators;

namespace Datasources.CodeGenerators.Tests;

public class NestedSelectTypesTests
{
    [Fact]
    public void BuildsContactsGraphFromSpec()
    {
        var spec = SpecParser.Parse(Files());
        var graph = NestedSelectTypes.FromSpec(spec.ExpandedTypes, "contacts_base");
        Assert.Equal("ContactsBase", graph.Name);
        Assert.Equal("Id", graph.Key);
        Assert.Contains("FirstName", graph.Properties);
        Assert.Contains(graph.Relations, item => item.Name == "ContactSource" && item.Kind == NestedSelectRelationKind.Object);
        Assert.Contains(graph.Relations, item => item.Name == "Addresses" && item.Kind == NestedSelectRelationKind.List);
        Assert.Contains(graph.Relations, item => item.Name == "Phones" && item.Kind == NestedSelectRelationKind.List);
        var nested = new NestedSelectBuilder().Build(graph);
        Assert.Contains(nested.Select.Joins, join => join.Table == "Address");
        Assert.Contains(nested.Select.Fields, field => field.Contains("FirstName", StringComparison.Ordinal));
    }

    static IReadOnlyDictionary<string, string> Files()
    {
        var dir = Path.Combine(SpecParser.SpecsDir(), "contacts");
        return new Dictionary<string, string>
        {
            [SpecParser.TypesYaml] = File.ReadAllText(Path.Combine(dir, "types.yaml")),
            [SpecParser.DatasourceYaml] = File.ReadAllText(Path.Combine(dir, "datasource.yaml")),
        };
    }
}
