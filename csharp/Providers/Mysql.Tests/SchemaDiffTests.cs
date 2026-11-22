using Datasources;
using Datasources.CodeGenerators;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class SchemaDiffTests
{
    static (IReadOnlyList<SqlTable> Tables, StatementNaming Naming) Snap(string yaml)
    {
        var spec = SpecParser.Parse(new Dictionary<string, string> { ["types.yaml"] = yaml });
        var types = SqlSchema.From(spec);
        var naming = PackCasing.Of(new Dictionary<string, string>()).Naming;
        return (SqlSchema.BuildLive(types, naming), naming);
    }

    const string IntegerAge = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields:
                - age:
                    type: integer
        """;

    const string StringAge = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields:
                - age:
                    type: string
        """;

    [Fact]
    public void KeepsAlterColumnUnlessRebuildTableIncludesMysql()
    {
        RegisterSqlGenerate.Register();
        var before = Snap(IntegerAge);
        var after = Snap(StringAge);
        var statements = SchemaDiff.Of("mysql", before.Tables, before.Naming, after.Tables, after.Naming);
        Assert.Contains(statements, statement => statement is AlterTableStatement);
        Assert.DoesNotContain(statements, statement => statement is RebuildTableStatement);
        var rebuilt = SchemaDiff.Of(
            "mysql",
            before.Tables,
            before.Naming,
            after.Tables,
            after.Naming,
            new Dictionary<string, string> { ["datasource.rebuild_table"] = "mysql" });
        Assert.Contains(rebuilt, statement => statement is RebuildTableStatement);
        Assert.DoesNotContain(rebuilt, statement => statement is AlterTableStatement);
        var sql = new StatementTextRenderer(t => new MysqlStatementRenderer(t)).Render(rebuilt.OfType<RebuildTableStatement>().Single());
        Assert.Contains("RENAME TABLE", sql);
        Assert.Contains("users__new", sql);
    }
}
