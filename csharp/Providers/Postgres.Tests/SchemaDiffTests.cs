using Datasources;
using Datasources.CodeGenerators;

namespace Datasources.Postgres.Tests;

public class SchemaDiffTests
{
    static (IReadOnlyList<SqlTable> Tables, StatementNaming Naming) Snap(string yaml)
    {
        var spec = SpecParser.Parse(new Dictionary<string, string> { ["types.yaml"] = yaml });
        var types = SqlSchema.From(spec);
        var naming = PackCasing.Of(new Dictionary<string, string>()).Naming;
        return (SqlSchema.BuildLive(types, naming), naming);
    }

    [Fact]
    public void KeepsAlterColumnWhenAColumnTypeChanges()
    {
        RegisterSqlGenerate.Register();
        var before = Snap("""
            types:
              - user:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - age:
                        type: integer
            """);
        var after = Snap("""
            types:
              - user:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - age:
                        type: string
            """);
        var statements = SchemaDiff.Of("postgres", before.Tables, before.Naming, after.Tables, after.Naming);
        Assert.Contains(statements, statement => statement is AlterTableStatement);
        Assert.DoesNotContain(statements, statement => statement is RebuildTableStatement);
    }
}
