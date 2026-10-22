using Datasources.CodeGenerators;

namespace Datasources.Sqlite.Tests;

public class GenerateSqlQuotesTests
{
    [Fact]
    public void KeywordsLowerKeepsSqliteQuotedObjectNames()
    {
        var files = new Dictionary<string, string>
        {
            ["types.yaml"] = """
                types:
                  - notification_type:
                      tags: [datasource_type]
                      inherits: set
                      fields:
                        - channel_name:
                            type: string
                """,
        };
        var sql = GenerateSql.Of(
            files,
            new Dictionary<string, string>
            {
                ["datasource.casing.keywords"] = "lower",
                ["datasource.casing.objects"] = "upper",
            },
            ["sqlite"]).Single(entry => entry.Filename == "sqlite/migrations/0001_initial_up.sql").Content;
        Assert.Contains("create table \"NOTIFICATION_TYPES\"", sql);
        Assert.Contains("not null", sql);
    }
}
