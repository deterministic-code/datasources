using Datasources;
using Datasources.CodeGenerators;

namespace Datasources.Sqlite.Tests;

public class SchemaDiffTests
{
    const string TypesYaml = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields:
                - email:
                    type: string
        """;

    static (IReadOnlyList<SqlTable> Tables, SqlMapping Mapping) Snap(string yaml)
    {
        var spec = SpecParser.Parse(new Dictionary<string, string> { ["types.yaml"] = yaml });
        var types = SqlSchema.From(spec);
        var mapping = new SqlMapping(PackCasing.Of(new Dictionary<string, string>()), types);
        return (SqlSchema.BuildLive(types, mapping), mapping);
    }

    [Fact]
    public void CreatesAnAddedTableAndDropsARemovedTable()
    {
        var (tables, mapping) = Snap(TypesYaml);
        var empty = Array.Empty<SqlTable>();
        Assert.Contains(SchemaDiff.Of("sqlite", empty, mapping, tables, mapping), statement => statement is CreateTableStatement);
        Assert.Contains(SchemaDiff.Of("sqlite", tables, mapping, empty, mapping), statement => statement is DropTableStatement);
    }
}
