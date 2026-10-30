using Datasources.CodeGenerators;

namespace Datasources.Sqlite.Tests;

public class GenerateSqlTests
{
    const string TypesYaml = """
        types:
          - user:
              tags: [datasource_type]
              inherits: set
              fields:
                - email:
                    type: string
                - created:
                    type: datetime
                - updated:
                    type: datetime
        """;

    const string DefaultDatasourceYaml = """
        version: 1.0.0
        includes:
          - types:
              filter: tag == "datasource_type" || tag == "view_type"
        types: []
        """;

    const string EngineDatasourceYaml = """
        version: 1.0.0
        types:
          - user:
              fields:
                - email:
                    type: string
        """;

    static Dictionary<string, string> Files(string types, string? datasource = null, string? seeds = null)
    {
        var files = new Dictionary<string, string> { ["types.yaml"] = types };
        if (datasource is not null) files["datasource.yaml"] = datasource;
        if (seeds is not null) files["datasource_seeds.yaml"] = seeds;
        return files;
    }

    static IReadOnlyList<string> Names(IReadOnlyList<GenerateEntry> entries) =>
        entries.Select(e => e.Filename).OrderBy(n => n, StringComparer.Ordinal).ToList();

    static string AddedTypes(string types) => $"types.yaml:\n  added: true\n  content:\n{Indent(types, 4)}";

    static string Indent(string yaml, int spaces)
    {
        var pad = new string(' ', spaces);
        return string.Join('\n', yaml.Trim().Split('\n').Select(line => pad + line)) + "\n";
    }

    [Fact]
    public void OmitsStoredProcedureMigrationsOnSqliteEvenWhenTheFlagIsTrue()
    {
        var files = Names(GenerateSql.Of(
            Files(TypesYaml),
            new Dictionary<string, string> { ["datasource.use_stored_procedures"] = "true" },
            ["sqlite"]));
        Assert.Contains("sqlite/migrations/0001_initial_up.sql", files);
        Assert.DoesNotContain(files, f => f.Contains("stored_procedures"));
    }

    static string UpSql(IReadOnlyDictionary<string, string> settings, string types = TypesYaml)
    {
        var up = GenerateSql.Of(Files(types), settings, ["sqlite"])
            .Single(entry => entry.Filename.EndsWith("sqlite/migrations/0001_initial_up.sql"));
        return up.Content;
    }

    [Fact]
    public void DefaultsToAWholeLineGoBetweenTableAndTrigger() =>
        Assert.Matches(@";\nGO\nCREATE TRIGGER ""trg_users_updated_at""", UpSql(new Dictionary<string, string>()));

    [Fact]
    public void PutsGoBetweenTables()
    {
        var sql = UpSql(new Dictionary<string, string>(), """
            types:
              - gender:
                  tags: [datasource_type]
                  fields:
                    - id:
                        type: integer
                        is_id: true
                    - created:
                        type: datetime
                    - updated:
                        type: datetime
              - contact:
                  tags: [datasource_type]
                  fields:
                    - id:
                        type: integer
                        is_id: true
                    - created:
                        type: datetime
                    - updated:
                        type: datetime
            """);
        Assert.Matches(@"trg_genders_updated_at[\s\S]*;\nGO\nCREATE TABLE ""contacts""", sql);
    }

    [Fact]
    public void UsesTheConfiguredSeparator()
    {
        var sql = UpSql(new Dictionary<string, string> { ["datasource.batch_separator"] = "BATCH" });
        Assert.Matches(@";\nBATCH\nCREATE TRIGGER ""trg_users_updated_at""", sql);
        Assert.DoesNotMatch(@"\nGO\n", sql);
    }

    [Fact]
    public void SeparatesSeedInsertsWithDatasourceBatchSeparator()
    {
        var sql = GenerateSql.Of(
            Files("""
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
                seeds: """
                version: 1.0.0
                seeds:
                  - gender:
                      - id1:
                          name: Male
                      - id2:
                          name: Female
                """),
            new Dictionary<string, string>(),
            ["sqlite"])
            .Single(entry => entry.Filename.EndsWith("sqlite/migrations/0001_initial_up.sql")).Content;
        Assert.Matches(@"INSERT INTO ""genders""[\s\S]*;\nGO\nINSERT INTO ""genders""", sql);
    }

    [Fact]
    public void ThrowsWhenIdIsMissingIsId()
    {
        var error = Assert.Throws<InvalidOperationException>(() => GenerateSql.Of(Files("""
            types:
              - card:
                  tags: [datasource_type]
                  fields:
                    - id:
                        type: integer
                    - label:
                        type: string
            """), new Dictionary<string, string>(), ["sqlite"]));
        Assert.Contains("card.id must have is_id: true", error.Message);
    }

    [Fact]
    public void OmitsViewCollectionsFromTableColumnsAndKeepsTheChildFk()
    {
        var entries = GenerateSql.Of(Files("""
            types:
              - contact:
                  tags: [datasource_type, view_type]
                  inherits: set
                  fields:
                    - email:
                        type: string
                    - addresses:
                        type: address[]
                        references: address.contact_id
              - address:
                  tags: [datasource_type, view_type]
                  inherits: set
                  fields:
                    - contact_id:
                        type: integer
                        references: contact.id
                    - city:
                        type: string
            """), new Dictionary<string, string>(), ["sqlite"]);
        var sql = entries.Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches("CREATE TABLE \"contacts\"", sql);
        Assert.Matches("CREATE TABLE \"addresses\"", sql);
        var contacts = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE ""contacts"" \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches("\"email\"", contacts);
        Assert.DoesNotMatch("\"addresses\"", contacts);
        Assert.Matches("\"contact_id\"", sql);
        Assert.Matches(@"FOREIGN KEY \(""contact_id""\) REFERENCES ""contacts""", sql);
    }

    [Fact]
    public void DefaultsUuidCreatedUpdatedAndVersionOnABaseTable()
    {
        var entries = GenerateSql.Of(Files("""
            types:
              - base:
                  fields:
                    - id:
                        type: integer
                        is_id: true
                    - uuid:
                        type: uuid
                        default_value: NewId
                    - created:
                        type: datetime
                        default_value: UtcNow
                    - updated:
                        type: datetime
                        default_value: UtcNow
                    - version:
                        type: binary
                        default_value: Hex('00')
              - card:
                  tags: [datasource_type]
                  inherits: base
                  fields:
                    - label:
                        type: string
            """), new Dictionary<string, string>(), ["sqlite"]);
        var sql = entries.Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        var cards = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE ""cards"" \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches(@"""uuid"" TEXT .*DEFAULT \(lower\(hex\(randomblob", cards);
        Assert.Matches(@"""created"" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'", cards);
        Assert.Matches(@"""updated"" TEXT .*strftime\('%Y-%m-%dT%H:%M:%fZ'", cards);
        Assert.Matches(@"""version"" BLOB .*DEFAULT X'00'", cards);
    }

    [Fact]
    public void InfersAReferenceColumnTypeFromTheParentId()
    {
        var entries = GenerateSql.Of(Files("""
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
            """), new Dictionary<string, string>(), ["sqlite"]);
        var sql = entries.Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        var contacts = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE ""contacts"" \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches(@"""gender_type_id"" INTEGER", contacts);
        Assert.DoesNotMatch(@"""gender_type_id"" TEXT", contacts);
    }

    [Fact]
    public void EmitsADictionaryTableWithUniqueOwnerAndKeyFlatteningObjectValues()
    {
        var entries = GenerateSql.Of(Files("""
            types:
              - contact:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - email:
                        type: string
              - locale_preferences:
                  tags: [view_type]
                  fields:
                    - locale:
                        type: string
                    - timezone:
                        type: string
              - contact_settings:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - contact_id:
                        type: integer
                        references: contact.id
                    - key:
                        type: string
                    - value:
                        type: string
              - contact_preferences:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - contact_id:
                        type: integer
                        references: contact.id
                    - key:
                        type: string
                    - locale:
                        type: string
                    - timezone:
                        type: string
              - contact_settings_map:
                  tags: [view_type]
                  inherits: dictionary
                  fields:
                    - key:
                        type: string
                    - value:
                        type: string
              - contact_preferences_map:
                  tags: [view_type]
                  inherits: dictionary
                  fields:
                    - key:
                        type: string
                    - value:
                        type: locale_preferences
              - contact_view:
                  tags: [view_type]
                  inherits: contact
                  fields:
                    - settings:
                        type: contact_settings_map{}
                        references: contact_settings.contact_id
                    - prefs:
                        type: contact_preferences_map{}
                        references: contact_preferences.contact_id
              - phone:
                  tags: [datasource_type]
                  inherits: set
              - phone_attribute:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - phone_id:
                        type: integer
                        references: phone.id
                    - attribute_name:
                        type: string
                    - attribute_value:
                        type: string
              - phone_attribute_map:
                  tags: [view_type]
                  inherits: dictionary
                  fields:
                    - key:
                        type: string
                        references: phone_attribute.attribute_name
                    - value:
                        type: string
                        references: phone_attribute.attribute_value
              - phone_view:
                  tags: [view_type]
                  inherits: phone
                  fields:
                    - attributes:
                        type: phone_attribute_map{}
                        references: phone_attribute.phone_id
              - card_labels:
                  tags: [view_type]
                  inherits: dictionary
                  fields:
                    - key:
                        type: string
                    - value:
                        type: string
            """), new Dictionary<string, string>(), ["sqlite"]);
        var sql = entries.Single(e => e.Filename.EndsWith("0001_initial_up.sql")).Content;
        Assert.Matches("CREATE TABLE \"contact_settings\"", sql);
        Assert.Matches("CREATE TABLE \"contact_preferences\"", sql);
        Assert.DoesNotMatch("CREATE TABLE \"card_labels\"", sql);
        var settings = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE ""contact_settings"" \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches("\"contact_id\"", settings);
        Assert.Matches("\"key\"", settings);
        Assert.Matches("\"value\"", settings);
        Assert.Matches(@"(?m)^\s*""id""", settings);
        var prefs = System.Text.RegularExpressions.Regex.Match(sql, @"CREATE TABLE ""contact_preferences"" \(([\s\S]*?)\);").Groups[1].Value;
        Assert.Matches("\"locale\"", prefs);
        Assert.Matches("\"timezone\"", prefs);
        Assert.DoesNotMatch("\"value\"", prefs);
        Assert.Matches(@"CREATE UNIQUE INDEX .* ON ""contact_settings""", sql);
        Assert.Matches(@"CREATE UNIQUE INDEX .* ON ""contact_preferences""", sql);
        Assert.Matches(@"CREATE UNIQUE INDEX .* ON ""phone_attributes"".*""phone_id"".*""attribute_name""", sql);
    }

    [Fact]
    public void EmitsNumberedUpDownFilesFromFrozenDiffs()
    {
        const string path = "version-20261016T000000Z-001/diff.yaml";
        var files = Files(TypesYaml, DefaultDatasourceYaml);
        files[path] = AddedTypes(TypesYaml);
        var names = Names(GenerateSql.Of(files, new Dictionary<string, string>(), ["sqlite"]));
        Assert.Equal(
            ["sqlite/migrations/0001_down.sql", "sqlite/migrations/0001_up.sql"],
            names);
    }

    [Fact]
    public void AcceptsADialectCoveredByDatasourceDialectYaml()
    {
        const string path = "version-20261016T000000Z-001/diff.yaml";
        var files = Files(TypesYaml);
        files["datasource.sqlite.yaml"] = EngineDatasourceYaml;
        files[path] = AddedTypes(TypesYaml);
        var names = Names(GenerateSql.Of(files, new Dictionary<string, string>(), ["sqlite"]));
        Assert.Equal(
            ["sqlite/migrations/0001_down.sql", "sqlite/migrations/0001_up.sql"],
            names);
    }

    [Fact]
    public void ThrowsWhenLiveYamlDriftedFromFrozenDiffs()
    {
        const string path = "version-20261016T000000Z-001/diff.yaml";
        var files = Files(TypesYaml, DefaultDatasourceYaml);
        files[path] = AddedTypes(TypesYaml);
        files["types.yaml"] = """
            types:
              - extra:
                  tags: [datasource_type]
                  inherits: set
                  fields:
                    - name:
                        type: string
            """;
        var error = Assert.Throws<InvalidOperationException>(() =>
            GenerateSql.Of(files, new Dictionary<string, string>(), ["sqlite"]));
        Assert.Contains("run datasources version first", error.Message);
    }

    [Fact]
    public void EmitsALaterVersionForAddedSeeds()
    {
        const string gender = """
            types:
              - gender:
                  tags: [datasource_type]
                  fields:
                    - id:
                        type: integer
                        is_id: true
                    - name:
                        type: string
            """;
        const string seeds = """
            version: 1.0.0
            seeds:
              - gender:
                  - id1:
                      name: Male
            """;
        const string first = "version-20261016T000000Z-001/diff.yaml";
        const string second = "version-20261016T000000Z-002/diff.yaml";
        var files = Files(gender, DefaultDatasourceYaml, seeds);
        files[first] = AddedTypes(gender);
        files[second] = AddedSeeds(seeds);
        var entries = GenerateSql.Of(files, new Dictionary<string, string>(), ["sqlite"]);
        Assert.Equal(
            ["sqlite/migrations/0001_down.sql", "sqlite/migrations/0001_up.sql", "sqlite/migrations/0002_down.sql", "sqlite/migrations/0002_up.sql"],
            Names(entries));
        Assert.Contains("INSERT INTO \"genders\"", entries.Single(e => e.Filename == "sqlite/migrations/0002_up.sql").Content);
        Assert.Contains("Male", entries.Single(e => e.Filename == "sqlite/migrations/0002_up.sql").Content);
        Assert.Contains("DELETE FROM \"genders\" WHERE \"id\" = 1", entries.Single(e => e.Filename == "sqlite/migrations/0002_down.sql").Content);
    }

    static string AddedSeeds(string seeds) => $"datasource_seeds.yaml:\n  added: true\n  content:\n{Indent(seeds, 4)}";

    [Fact]
    public void ThrowsWhenARootDatasourceYamlDriftedFromTheFrozenDiffs()
    {
        const string path = "version-20261016T000000Z-001/diff.yaml";
        var files = Files(TypesYaml, EngineDatasourceYaml);
        files[path] = AddedTypes(TypesYaml);
        var error = Assert.Throws<InvalidOperationException>(() =>
            GenerateSql.Of(files, new Dictionary<string, string>(), ["sqlite"]));
        Assert.Contains("run datasources version first", error.Message);
    }
}
