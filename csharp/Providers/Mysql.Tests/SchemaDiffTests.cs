using Datasources;
using Datasources.CodeGenerators;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class SchemaDiffTests
{
    static (IReadOnlyList<SqlTable> Tables, StatementNaming Naming) Snap(string yaml, string? datasourceYaml = null)
    {
        var files = new Dictionary<string, string> { ["types.yaml"] = yaml };
        if (datasourceYaml is not null)
            files["datasource.yaml"] = datasourceYaml;
        var spec = SpecParser.Parse(files);
        var types = SqlSchema.From(spec);
        var naming = PackCasing.Of(new Dictionary<string, string>()).Naming;
        return (SqlSchema.BuildLive(types, naming), naming);
    }

    static string Sql(IReadOnlyList<IStatement> statements) =>
        string.Join("\n", statements.Select(statement => new StatementTextRenderer(t => new MysqlStatementRenderer(t)).Render(statement)));

    const string UsersAndDevices = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields: []
          - device:
              tags: [datasource_type]
              inherits: set
              fields:
                - user_id:
                    type: integer
                    references: user.id
                - fingerprint:
                    type: string
        """;

    const string UsersAndDevicesNoFk = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields: []
          - device:
              tags: [datasource_type]
              inherits: set
              fields:
                - user_id:
                    type: integer
                - fingerprint:
                    type: string
        """;

    const string UsersOnly = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields: []
        """;

    const string DeviceFingerprintIndex = """
        includes:
          - types:
              filter: tag == "datasource_type"
        types:
          - device:
              indexes:
                - user_id_fingerprint:
                    fields: [user_id, fingerprint]
                    is_unique: true
        """;

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

    [Fact]
    public void RebuildsAMysqlAuditTableTriggerWhenAForeignKeyTargetChanges()
    {
        RegisterSqlGenerate.Register();
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
                    - created:
                        type: datetime
                    - updated:
                        type: datetime
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
                    - created:
                        type: datetime
                    - updated:
                        type: datetime
                    - user_id:
                        type: integer
                        references: account.id
            """);
        var rebuilt = SchemaDiff.Of(
            "mysql",
            before.Tables,
            before.Naming,
            after.Tables,
            after.Naming,
            new Dictionary<string, string> { ["datasource.rebuild_table"] = "mysql" });
        var sql = rebuilt.Select(statement => new StatementTextRenderer(t => new MysqlStatementRenderer(t)).Render(statement)).ToList();
        Assert.Contains(sql, item => item.Contains("RENAME TABLE") && item.Contains("orders__new"));
        Assert.Contains(sql, item => item.Contains("CREATE TRIGGER"));
    }

    [Fact]
    public void DropsARemovedTableWithoutDroppingItsIndexes()
    {
        RegisterSqlGenerate.Register();
        var before = Snap(UsersAndDevices, DeviceFingerprintIndex);
        var after = Snap(UsersOnly);
        var sql = Sql(SchemaDiff.Of("mysql", before.Tables, before.Naming, after.Tables, after.Naming));
        Assert.Contains("DROP TABLE", sql);
        Assert.DoesNotContain("DROP INDEX", sql);
    }

    [Fact]
    public void DropsAForeignKeyBeforeAnIndexThatSupportedIt()
    {
        RegisterSqlGenerate.Register();
        var before = Snap(UsersAndDevices, DeviceFingerprintIndex);
        var after = Snap(UsersAndDevicesNoFk);
        var sql = Sql(SchemaDiff.Of("mysql", before.Tables, before.Naming, after.Tables, after.Naming));
        Assert.Contains("DROP FOREIGN KEY", sql);
        Assert.Contains("DROP INDEX", sql);
        Assert.True(sql.IndexOf("DROP FOREIGN KEY", StringComparison.Ordinal) < sql.IndexOf("DROP INDEX", StringComparison.Ordinal));
    }

    [Fact]
    public void CreatesASupportingIndexBeforeDroppingACompositeIndexUsedByAForeignKey()
    {
        RegisterSqlGenerate.Register();
        var before = Snap(UsersAndDevices, DeviceFingerprintIndex);
        var after = Snap(UsersAndDevices);
        var sql = Sql(SchemaDiff.Of("mysql", before.Tables, before.Naming, after.Tables, after.Naming));
        Assert.Contains("CREATE INDEX", sql);
        Assert.Contains("DROP INDEX", sql);
        Assert.True(sql.IndexOf("CREATE INDEX", StringComparison.Ordinal) < sql.IndexOf("DROP INDEX", StringComparison.Ordinal));
        Assert.Matches(@"\(`?user_id`?\)", sql[..sql.IndexOf("DROP INDEX", StringComparison.Ordinal)]);
    }
}
