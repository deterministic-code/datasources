using System.Text.RegularExpressions;
using Datasources;
using Datasources.CodeGenerators;

namespace Datasources.Mysql.Tests;

public class GenerateSqlTests
{
    public GenerateSqlTests() => RegisterSqlGenerate.Register();

    [Fact]
    public void InfersAReferenceColumnTypeFromTheParentId()
    {
        var entries = GenerateSql.Of(new Dictionary<string, string>
        {
            ["types.yaml"] = """
                types:
                  - gender:
                      tags: [datasource_type]
                      inherits: set
                      fields:
                        - name:
                            type: string
                            size: 128
                  - contact:
                      tags: [datasource_type]
                      inherits: set
                      fields:
                        - gender_type_id:
                            references: gender.id
                """,
        }, new Dictionary<string, string>(), ["mysql"]);
        var sql = entries.Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        var contacts = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE contacts \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches(@"gender_type_id INT", contacts);
        Assert.DoesNotMatch(@"`gender_type_id` TEXT", contacts);
    }

    [Fact]
    public void SeparatesTableAndTriggerWithDatasourceBatchSeparator()
    {
        var files = new Dictionary<string, string>
        {
            ["types.yaml"] = """
                types:
                  - gender:
                      tags: [datasource_type]
                      fields:
                        - id:
                            type: integer
                            is_id: true
                        - created:
                            type: datetime
                            default_value: UtcNow
                        - updated:
                            type: datetime
                            default_value: UtcNow
                """,
        };
        var sql = GenerateSql.Of(files, new Dictionary<string, string>(), ["mysql"])
            .Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches(@";\nGO\nCREATE TRIGGER trg_genders_updated_at", sql);
        var custom = GenerateSql.Of(files, new Dictionary<string, string> { ["datasource.batch_separator"] = "BATCH" }, ["mysql"])
            .Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches(@";\nBATCH\nCREATE TRIGGER trg_genders_updated_at", custom);
        Assert.DoesNotMatch(@"\nGO\n", custom);
    }

    [Fact]
    public void SeparatesSeedInsertsWithDatasourceBatchSeparator()
    {
        var sql = GenerateSql.Of(new Dictionary<string, string>
        {
            ["types.yaml"] = """
                types:
                  - gender:
                      tags: [datasource_type]
                      fields:
                        - id:
                            type: integer
                            is_id: true
                        - name:
                            type: string
                """,
            ["datasource_seeds.yaml"] = """
                version: 1.0.0
                seeds:
                  - gender:
                      - id1:
                          name: Male
                      - id2:
                          name: Female
                """,
        }, new Dictionary<string, string>(), ["mysql"])
            .Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches(@"INSERT INTO genders[\s\S]*;\nGO\nINSERT INTO genders", sql);
    }

    [Fact]
    public void ShortensIndexForeignKeyAndTriggerNamesOver64CharactersInMigrations()
    {
        const string entity = "game_journey_setting_category_preference_option_value";
        var naming = PackCasing.Of(new Dictionary<string, string>()).Naming;
        var index = CreateIndexStatement.NameOf(entity, naming, "category_type_id", "index");
        var foreignKey = ForeignKey.NameOf(entity, naming, "category_type_id", "foreign_key");
        var trigger = CreateTriggerStatement.NameOf(entity, naming);
        var primaryKey = PrimaryKey.NameOf(entity, naming, "primary_key");
        foreach (var name in new[] { index, foreignKey, trigger, primaryKey })
            Assert.True(name.Length > 64, name);
        var files = new Dictionary<string, string>
        {
            ["types.yaml"] = $$"""
                types:
                  - category_type:
                      tags: [datasource_type]
                      fields:
                        - id:
                            type: integer
                            is_id: true
                  - {{entity}}:
                      tags: [datasource_type]
                      fields:
                        - id:
                            type: integer
                            is_id: true
                        - created:
                            type: datetime
                        - updated:
                            type: datetime
                        - category_type_id:
                            type: integer
                            references: category_type.id
                        - external_reference_code:
                            type: string
                """,
            ["datasource.yaml"] = $$"""
                includes:
                  - types:
                      filter: tag == "datasource_type"
                types:
                  - {{entity}}:
                      indexes:
                        - category_type_id:
                            fields:
                              - category_type_id
                """,
        };
        var entries = GenerateSql.Of(files, new Dictionary<string, string>(), ["mysql"]);
        var up = entries.Single(e => e.Filename.EndsWith("mysql/migrations/0001_initial_up.sql")).Content;
        var down = entries.Single(e => e.Filename.EndsWith("mysql/migrations/0001_initial_down.sql")).Content;
        var indexName = MysqlIdentifier.Fit(index);
        var foreignName = MysqlIdentifier.Fit(foreignKey);
        var triggerName = MysqlIdentifier.Fit(trigger);
        var primaryName = MysqlIdentifier.Fit(primaryKey);
        Assert.True(indexName.Length <= 64);
        Assert.NotEqual(index, indexName);
        Assert.Matches($@"CREATE INDEX (`?{Regex.Escape(indexName)}`?) ON", up);
        Assert.Matches($@"CONSTRAINT (`?{Regex.Escape(foreignName)}`?) FOREIGN KEY", up);
        Assert.Matches($@"CONSTRAINT (`?{Regex.Escape(primaryName)}`?) PRIMARY KEY", up);
        Assert.Matches($@"CREATE TRIGGER (`?{Regex.Escape(triggerName)}`?)", up);
        foreach (var name in new[] { index, foreignKey, trigger, primaryKey })
        {
            Assert.DoesNotContain(name, up);
            Assert.DoesNotContain(name, down);
        }
        Assert.Contains(indexName, up);
        Assert.Contains(foreignName, up);
        Assert.Contains(triggerName, up);
        Assert.Contains(primaryName, up);
    }
}
