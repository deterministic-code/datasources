using Datasources.CodeGenerators;

namespace Datasources.CodeGenerators.Tests;

public class WriteInclusionTests
{
    [Fact]
    public void DefaultsOmitASingleGeneratedId()
    {
        var (create, update) = WriteInclusion.Resolve(null, null, identity: true, fixedId: false, singleGeneratedId: true);
        Assert.False(create);
        Assert.False(update);
    }

    [Fact]
    public void DefaultsIncludeAFixedIdOnCreate()
    {
        var (create, update) = WriteInclusion.Resolve(null, null, identity: true, fixedId: true, singleGeneratedId: false);
        Assert.True(create);
        Assert.False(update);
    }

    [Fact]
    public void DefaultsIncludeOrdinaryFields()
    {
        var (create, update) = WriteInclusion.Resolve(null, null, identity: false, fixedId: false, singleGeneratedId: false);
        Assert.True(create);
        Assert.True(update);
    }

    [Fact]
    public void AuthoredFlagsWin()
    {
        var (create, update) = WriteInclusion.Resolve(false, false, identity: false, fixedId: false, singleGeneratedId: false);
        Assert.False(create);
        Assert.False(update);
    }

    [Fact]
    public void ParserMergesInheritedDatasourceOverlays()
    {
        var spec = SpecParser.Parse(new Dictionary<string, string>
        {
            ["types.yaml"] = """
                types:
                  - base:
                      fields:
                        - id:
                            type: integer
                            is_id: true
                        - created:
                            type: datetime
                  - item:
                      tags:
                        - datasource_type
                      inherits: base
                      fields:
                        - name:
                            type: string
                """,
            ["datasource.yaml"] = """
                includes:
                  - types:
                      filter: tag == "datasource_type"
                types:
                  - base:
                      fields:
                        - created:
                            include_in_create: false
                            include_in_update: false
                """,
        });
        var item = spec.ExpandedTypes.Single(type => type.Name == "item");
        var created = item.Fields.Single(field => field.Name == "created");
        var name = item.Fields.Single(field => field.Name == "name");
        var table = spec.Datasource.Single(t => t.Name == "item");
        Assert.False(WriteInclusion.Create(created, item, table));
        Assert.False(WriteInclusion.Update(created, item, table));
        Assert.True(WriteInclusion.Create(name, item, table));
        Assert.True(WriteInclusion.Update(name, item, table));
    }
}
