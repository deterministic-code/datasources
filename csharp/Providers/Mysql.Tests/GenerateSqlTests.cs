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
}
