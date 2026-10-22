using Datasources.CodeGenerators;

namespace Datasources.Postgres.Tests;

public class GenerateSqlMappingTests
{
    const string Types = """
        types:
          - users_base:
              tags: [datasource_type]
              inherits: set
              fields:
                - email:
                    type: string
                - created:
                    type: datetime
                - updated:
                    type: datetime
          - child:
              tags: [datasource_type]
              inherits: set
              fields:
                - owner_id:
                    type: integer
                    references: users_base.id
                - email:
                    type: string
                - created:
                    type: datetime
                - updated:
                    type: datetime
          - person:
              tags: [datasource_type]
              inherits: set
              fields:
                - name:
                    type: string
          - old_reviews:
              tags: [datasource_type]
              inherits: set
              fields:
                - title:
                    type: string
          - notification:
              tags: [datasource_type]
              inherits: set
              fields:
                - title:
                    type: string
        """;

    const string Datasource = """
        includes:
          - types:
              filter: tag == "datasource_type"
        types:
          - users_base:
              mapping: user
          - child:
              fields:
                - email:
                    mapping: email_address
          - old_reviews:
              mapping: OldRvwsTbl
        """;

    const string Seeds = """
        seeds:
          - child:
              - id1:
                  email: mapped@example.com
                  owner_id: 1
        """;

    static string UpSql(IReadOnlyDictionary<string, string>? settings = null)
    {
        var files = new Dictionary<string, string>
        {
            ["types.yaml"] = Types,
            ["datasource.yaml"] = Datasource,
            ["datasource_seeds.yaml"] = Seeds,
        };
        return GenerateSql.Of(files, settings ?? new Dictionary<string, string>(), ["postgres"])
            .Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
    }

    [Fact]
    public void PluralizesASnakeTableMapping()
    {
        var sql = UpSql();
        Assert.Matches("CREATE TABLE \"users\"", sql);
        Assert.Matches("CONSTRAINT \"users_primary_key\"", sql);
        Assert.Matches("trg_users_updated_at", sql);
        Assert.DoesNotMatch("users_base", sql);
        Assert.DoesNotMatch("users_bases", sql);
    }

    [Fact]
    public void KeepsASnakeTableMappingSingularWhenPluralizeIsOff()
    {
        var sql = UpSql(new Dictionary<string, string> { ["datasource.pluralize_datatable_names"] = "false" });
        Assert.Matches("CREATE TABLE \"user\"", sql);
        Assert.Matches("CONSTRAINT \"user_primary_key\"", sql);
        Assert.Matches(@"REFERENCES ""user""\s*\(""id""\)", sql);
    }

    [Fact]
    public void PluralizesAnUnmappedTypeName()
    {
        var sql = UpSql();
        Assert.Matches("CREATE TABLE \"people\"", sql);
        Assert.Matches("CREATE TABLE \"notifications\"", sql);
    }

    [Fact]
    public void EmitsMappedColumnsAndSeedInsertLists()
    {
        var sql = UpSql();
        Assert.Matches(@"CREATE TABLE ""children""[\s\S]*""email_address""", sql);
        Assert.Matches(@"INSERT INTO ""children"" \(""id"", ""email_address"", ""owner_id""\)", sql);
        Assert.DoesNotMatch(@"CREATE TABLE ""children""[\s\S]*""email""[^_]", sql);
    }

    [Fact]
    public void ResolvesLogicalFkReferencesToTheTargetPhysicalName()
    {
        Assert.Matches(@"REFERENCES ""users""\s*\(""id""\)", UpSql());
    }

    [Fact]
    public void KeepsANonSnakeMappingVerbatimWhenPluralizeIsOn()
    {
        var sql = UpSql();
        Assert.Matches("CREATE TABLE \"OldRvwsTbl\"", sql);
        Assert.DoesNotMatch("OldRvwsTbls", sql);
    }
}
