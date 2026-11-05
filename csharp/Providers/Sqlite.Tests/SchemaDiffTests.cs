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

    static (IReadOnlyList<SqlTable> Tables, StatementNaming Naming) Snap(string yaml)
    {
        var spec = SpecParser.Parse(new Dictionary<string, string> { ["types.yaml"] = yaml });
        var types = SqlSchema.From(spec);
        var naming = PackCasing.Of(new Dictionary<string, string>()).Naming;
        return (SqlSchema.BuildLive(types, naming), naming);
    }

    [Fact]
    public void CreatesAnAddedTableAndDropsARemovedTable()
    {
        var (tables, naming) = Snap(TypesYaml);
        var empty = Array.Empty<SqlTable>();
        Assert.Contains(SchemaDiff.Of("sqlite", empty, naming, tables, naming), statement => statement is CreateTableStatement);
        Assert.Contains(SchemaDiff.Of("sqlite", tables, naming, empty, naming), statement => statement is DropTableStatement);
    }

    [Fact]
    public void RebuildsASqliteTableWhenAForeignKeyTargetChanges()
    {
        var before = Snap("""
            types:
              - user:
                  tags: [datasource_type]
                  inherits: set
                  fields: []
              - order:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - user_id:
                        type: integer
                        references: user.id
            """);
        var after = Snap("""
            types:
              - account:
                  tags: [datasource_type]
                  inherits: set
                  fields: []
              - order:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - user_id:
                        type: integer
                        references: account.id
            """);
        var renderer = new StatementTextRenderer(t => new SqliteStatementRenderer(t));
        var sql = SchemaDiff.Of("sqlite", before.Tables, before.Naming, after.Tables, after.Naming)
            .Select(statement => renderer.Render(statement))
            .ToList();
        Assert.Contains(sql, item => item.Contains("INSERT INTO") && item.Contains("orders__new"));
        Assert.Contains(sql, item => item.Contains("accounts"));
    }
}
