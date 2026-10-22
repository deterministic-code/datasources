using Datasources.CodeGenerators;

namespace Datasources.Postgres.Tests;

public class GenerateSqlCasingTests
{
    const string FixtureYaml = """
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
          - notification_type:
              tags: [datasource_type]
              inherits: set
              fields:
                - channel_name:
                    type: string
                - user_id:
                    type: integer
                    references: user.id
                - created:
                    type: datetime
                    default_value: UtcNow
                - updated:
                    type: datetime
                    default_value: UtcNow
        """;

    const string SeedsYaml = """
        seeds:
          - user:
              - id1:
                  email: CREATE
        """;

    static Dictionary<string, string> Files(bool withSeeds = false)
    {
        var files = new Dictionary<string, string> { ["types.yaml"] = FixtureYaml };
        if (withSeeds) files["datasource_seeds.yaml"] = SeedsYaml;
        return files;
    }

    static string FileSql(string filename, IReadOnlyDictionary<string, string> settings, bool withSeeds = false, IReadOnlyList<string>? dialects = null)
    {
        var entries = GenerateSql.Of(Files(withSeeds), settings, dialects ?? ["postgres"]);
        return entries.Single(e => e.Filename == filename).Content;
    }

    static string UpSql(IReadOnlyDictionary<string, string>? settings = null) =>
        FileSql("postgres/migrations/0001_initial_up.sql", settings ?? new Dictionary<string, string>());

    static string ProcSql(IReadOnlyDictionary<string, string> settings) =>
        FileSql("postgres/migrations/0002_stored_procedures_up.sql", new Dictionary<string, string>(settings) { ["datasource.use_stored_procedures"] = "true" });

    [Fact]
    public void AutoPlusPluralizeUsesSnakePluralTablesAndSnakeColumns()
    {
        var sql = UpSql();
        Assert.Matches("CREATE TABLE \"notification_types\"", sql);
        Assert.Matches("CREATE TABLE \"users\"", sql);
        Assert.Matches("\"channel_name\"", sql);
        Assert.Matches("\"notification_types_user_id_foreign_key\"", sql);
        Assert.Matches(@"REFERENCES ""users""\s*\(""id""\)", sql);
    }

    public static IEnumerable<object[]> TypesMatrix()
    {
        var types = new Dictionary<string, Dictionary<string, (string notificationTable, string userTable, string constraint)>>
        {
            ["on"] = new()
            {
                ["Camel"] = ("notificationTypes", "users", "notificationTypesUserIdForeignKey"),
                ["Pascal"] = ("NotificationTypes", "Users", "NotificationTypesUserIdForeignKey"),
                ["Snake"] = ("notification_types", "users", "notification_types_user_id_foreign_key"),
                ["Kebab"] = ("notification-types", "users", "notification-types-user-id-foreign-key"),
            },
            ["off"] = new()
            {
                ["Camel"] = ("notificationType", "user", "notificationTypeUserIdForeignKey"),
                ["Pascal"] = ("NotificationType", "User", "NotificationTypeUserIdForeignKey"),
                ["Snake"] = ("notification_type", "user", "notification_type_user_id_foreign_key"),
                ["Kebab"] = ("notification-type", "user", "notification-type-user-id-foreign-key"),
            },
        };
        var routines = new Dictionary<string, (string createFn, string findAllFn)>
        {
            ["Camel"] = ("createNotificationTypes", "readNotificationTypes"),
            ["Pascal"] = ("CreateNotificationTypes", "ReadNotificationTypes"),
            ["Snake"] = ("create_notification_types", "read_notification_types"),
            ["Kebab"] = ("create-notification-types", "read-notification-types"),
        };
        foreach (var format in new[] { "Camel", "Pascal", "Snake", "Kebab" })
        foreach (var pluralize in new[] { "on", "off" })
            yield return [format, pluralize, types[pluralize][format], routines[format]];
    }

    [Theory]
    [MemberData(nameof(TypesMatrix))]
    public void TypesCasingMatrix(string format, string pluralize, (string notificationTable, string userTable, string constraint) expected, (string createFn, string findAllFn) routines)
    {
        var settings = new Dictionary<string, string> { ["datasource.casing.types"] = format };
        if (pluralize == "off") settings["datasource.pluralize_datatable_names"] = "false";
        var sql = UpSql(settings);
        Assert.Matches($"CREATE TABLE \"{expected.notificationTable}\"", sql);
        Assert.Matches($"CREATE TABLE \"{expected.userTable}\"", sql);
        Assert.Matches("\"channel_name\"", sql);
        Assert.Matches($"\"{expected.constraint}\"", sql);
        Assert.Matches($@"REFERENCES ""{expected.userTable}""\s*\(""id""\)", sql);
        if (pluralize == "off") Assert.DoesNotMatch("CREATE TABLE \"notification_types\"", sql);
        var procs = ProcSql(settings);
        Assert.Matches($"CREATE OR REPLACE FUNCTION {routines.createFn}\\(", procs);
        Assert.Matches($"CREATE OR REPLACE FUNCTION {routines.findAllFn}\\(", procs);
        Assert.Matches($"INSERT INTO \"{expected.notificationTable}\"", procs);
    }

    [Theory]
    [InlineData("Camel", "channelName", "userId", "id")]
    [InlineData("Pascal", "ChannelName", "UserId", "Id")]
    [InlineData("Snake", "channel_name", "user_id", "id")]
    [InlineData("Kebab", "channel-name", "user-id", "id")]
    public void FieldsCasingColumnsOnly(string format, string channel, string userId, string id)
    {
        var sql = UpSql(new Dictionary<string, string> { ["datasource.casing.fields"] = format });
        Assert.Matches("CREATE TABLE \"notification_types\"", sql);
        Assert.Matches($"\"{channel}\"", sql);
        Assert.Matches($"\"{userId}\"", sql);
        Assert.Matches($@"REFERENCES ""users""\s*\(""{id}""\)", sql);
    }

    [Fact]
    public void PascalTypesAndCamelFieldsApplyIndependently()
    {
        var settings = new Dictionary<string, string>
        {
            ["datasource.casing.types"] = "Pascal",
            ["datasource.casing.fields"] = "Camel",
        };
        var sql = UpSql(settings);
        Assert.Matches("CREATE TABLE \"NotificationTypes\"", sql);
        Assert.Matches("\"channelName\"", sql);
        Assert.Matches("\"NotificationTypesUserIdForeignKey\"", sql);
        Assert.Matches(@"REFERENCES ""Users""\s*\(""id""\)", sql);
        var procs = ProcSql(settings);
        Assert.Matches(@"CREATE OR REPLACE FUNCTION CreateNotificationTypes\(", procs);
        Assert.Matches("channelName", procs);
    }

    [Fact]
    public void LanguagesSqlCasingDoesNotChangeEmit()
    {
        var sql = UpSql(new Dictionary<string, string>
        {
            ["languages.sql.casing.types"] = "Pascal",
            ["languages.sql.casing.fields"] = "Camel",
            ["languages.sql.casing.keywords"] = "lower",
        });
        Assert.Matches("CREATE TABLE \"notification_types\"", sql);
        Assert.Matches("\"channel_name\"", sql);
        Assert.DoesNotMatch("CREATE TABLE \"NotificationTypes\"", sql);
        Assert.DoesNotMatch("create table", sql);
    }

    [Fact]
    public void KeywordsLowerLowercasesSqlKeywordsAndTypes()
    {
        var sql = UpSql(new Dictionary<string, string> { ["datasource.casing.keywords"] = "lower" });
        Assert.Matches("create table \"notification_types\"", sql);
        Assert.Matches("not null", sql);
        Assert.Matches(@"references ""users""\s*\(""id""\)", sql);
        Assert.Matches("\"email\" text not null", sql);
        Assert.DoesNotMatch("CREATE TABLE", sql);
        var procs = ProcSql(new Dictionary<string, string> { ["datasource.casing.keywords"] = "lower" });
        Assert.Matches(@"create or replace function create_notification_types\(", procs);
        Assert.DoesNotMatch("CREATE OR REPLACE", procs);
    }

    [Fact]
    public void ObjectsUpperScreamsTableConstraintAndRoutineNames()
    {
        var sql = UpSql(new Dictionary<string, string> { ["datasource.casing.objects"] = "upper" });
        Assert.Matches("CREATE TABLE \"NOTIFICATION_TYPES\"", sql);
        Assert.Matches("CREATE TABLE \"USERS\"", sql);
        Assert.Matches("\"channel_name\"", sql);
        Assert.DoesNotMatch("CREATE TABLE \"notification_types\"", sql);
        var procs = ProcSql(new Dictionary<string, string> { ["datasource.casing.objects"] = "upper" });
        Assert.Matches(@"CREATE OR REPLACE FUNCTION CREATE_NOTIFICATION_TYPES\(", procs);
    }

    [Fact]
    public void ThrowsWhenKeywordsOrObjectsIsUnknown()
    {
        var keywords = Assert.Throws<InvalidOperationException>(() =>
            UpSql(new Dictionary<string, string> { ["datasource.casing.keywords"] = "screaming" }));
        Assert.Matches(@"datasource\.casing\.keywords must be one of \[upper, lower\]", keywords.Message);
        var objects = Assert.Throws<InvalidOperationException>(() =>
            UpSql(new Dictionary<string, string> { ["datasource.casing.objects"] = "screaming" }));
        Assert.Matches(@"datasource\.casing\.objects must be one of \[upper, lower\]", objects.Message);
    }

    [Fact]
    public void LeavesSeedStringLiteralsWhenKeywordsAreLower()
    {
        var sql = FileSql(
            "postgres/migrations/0001_initial_up.sql",
            new Dictionary<string, string> { ["datasource.casing.keywords"] = "lower" },
            true);
        Assert.Matches("insert into \"users\"", sql);
        Assert.Matches("'CREATE'", sql);
        Assert.DoesNotMatch("'create'", sql);
    }

    [Theory]
    [InlineData("postgres", "\"", "\"")]
    public void KeywordsLowerKeepsQuotedObjectNames(string dialect, string left, string right)
    {
        var body = FileSql(
            $"{dialect}/migrations/0001_initial_up.sql",
            new Dictionary<string, string>
            {
                ["datasource.casing.keywords"] = "lower",
                ["datasource.casing.objects"] = "upper",
            },
            dialects: [dialect]);
        Assert.Contains($"create table {left}NOTIFICATION_TYPES{right}", body);
        Assert.Matches("not null", body);
    }
}
