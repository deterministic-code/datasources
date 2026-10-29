using Datasources.CodeGenerators;

namespace Datasources.Mysql.Tests;

public class GenerateSqlTests
{
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
        var contacts = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE `contacts` \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches(@"`gender_type_id` INT", contacts);
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
        Assert.Matches(@";\nGO\nCREATE TRIGGER `trg_genders_updated_at`", sql);
        var custom = GenerateSql.Of(files, new Dictionary<string, string> { ["datasource.batch_separator"] = "BATCH" }, ["mysql"])
            .Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches(@";\nBATCH\nCREATE TRIGGER `trg_genders_updated_at`", custom);
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
        Assert.Matches(@"INSERT INTO `genders`[\s\S]*;\nGO\nINSERT INTO `genders`", sql);
    }
}
