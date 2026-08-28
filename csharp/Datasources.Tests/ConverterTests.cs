using Datasources;

namespace Datasources.Tests;

public class ConverterTests
{
    public ConverterTests() => TypeConverter.Reset();

    public class User
    {
        public int Id { get; set; }
        public string Username { get; set; } = "";
        public bool IsEnabled { get; set; }
        public DateTime Created { get; set; }
    }

    public class Mixed
    {
        public string Label { get; set; } = "";
        public long Count { get; set; }
        public double Ratio { get; set; }
        public decimal Amount { get; set; }
        public byte[] Data { get; set; } = [];
        public Guid Key { get; set; }
        public int? Optional { get; set; }
        public object Extra { get; set; } = new();
        public byte Tiny { get; set; }
        public float Real { get; set; }
        public string XMLParser { get; set; } = "";
        public int User2Name { get; set; }
        public string HTTP { get; set; } = "";
        public int this[int i] { get => 0; set { } }
    }

    public class WriteOnly
    {
        public int Id { get; set; }
        public int Secret { set { } }
    }

    [Fact]
    public void InfersUserSchemaAndRoundTrips()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<User>("sqlite");
        Assert.Equal("sqlite", mapping.Source);
        Assert.Equal("csharp", mapping.Destination);
        Assert.Equal("INTEGER", mapping.Schema["id"]);
        Assert.Equal("TEXT", mapping.Schema["username"]);
        Assert.Equal("BOOLEAN", mapping.Schema["is_enabled"]);
        Assert.Equal("DATETIME", mapping.Schema["created"]);
        var user = mapping.To(new Dictionary<string, object?>
        {
            ["id"] = 7,
            ["username"] = "ada",
            ["is_enabled"] = 1,
            ["created"] = "2020-01-02T03:04:05.000Z"
        });
        Assert.Equal(7, user.Id);
        Assert.Equal("ada", user.Username);
        Assert.True(user.IsEnabled);
        Assert.Equal(2020, user.Created.Year);
        var row = mapping.From(user);
        Assert.Equal(7, row["id"]);
        Assert.Equal("ada", row["username"]);
        Assert.Equal(1, row["is_enabled"]);
        Assert.Contains("2020-01-02", (string)row["created"]!);
    }

    [Fact]
    public void MergesSchemaAndConverters()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var twice = new FieldConverter(v => v is int n ? n * 2 : v, v => v is int n ? n / 2 : v);
        var mapping = TypeConverter.Mapping<User>("sqlite", schema: new Dictionary<string, string> { ["username"] = "VARCHAR(32)" }, converters: new Dictionary<string, IFieldConverter> { ["id"] = twice }, names: new Dictionary<string, string> { ["id"] = "Id", ["username"] = "Username" });
        Assert.Equal("VARCHAR(32)", mapping.Schema["username"]);
        Assert.Equal(6, mapping.To(new Dictionary<string, object?> { ["id"] = 3, ["username"] = "x" }).Id);
        Assert.Equal(3, mapping.From(new User { Id = 6, Username = "x" })["id"]);
    }

    [Fact]
    public void MapsDictionaryWithoutInferredSchema()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<Dictionary<string, object?>>("sqlite");
        Assert.Empty(mapping.Schema);
        var dest = mapping.To(new Dictionary<string, object?> { ["display_name"] = "a" });
        Assert.Equal("a", dest["DisplayName"]);
        Assert.Equal("a", mapping.From(new Dictionary<string, object?> { ["DisplayName"] = "a" })["display_name"]);
        var schemaOnly = TypeConverter.Mapping<Dictionary<string, object?>>(
            "sqlite",
            schema: new Dictionary<string, string> { ["display_name"] = "TEXT", ["___"] = "TEXT", [""] = "TEXT" });
        var converted = schemaOnly.To(new Dictionary<string, object?> { ["display_name"] = "a", ["___"] = "x", [""] = "z" });
        Assert.Equal("a", converted["displayName"]);
        TypeConverter.Use("json", TypeConverter.IdentityProviderTypes());
        var json = TypeConverter.Mapping<Dictionary<string, object?>>(
            "json",
            schema: new Dictionary<string, string> { ["keep"] = "TEXT" });
        Assert.Equal("a", json.To(new Dictionary<string, object?> { ["keep"] = "a" })["keep"]);
    }

    [Fact]
    public void IdentityProviderKeepsKeys()
    {
        TypeConverter.Use("json", TypeConverter.IdentityProviderTypes());
        var mapping = TypeConverter.Mapping<User>("json");
        Assert.Equal("BOOLEAN", mapping.Schema["IsEnabled"]);
        Assert.True(mapping.To(new Dictionary<string, object?> { ["IsEnabled"] = true }).IsEnabled);
    }

    [Fact]
    public void RejectsWrongDestinationAndMissingProvider()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        Assert.Throws<DatasourceException>(() => TypeConverter.Mapping<User>("sqlite", "typescript"));
        TypeConverter.Reset();
        Assert.Throws<DatasourceException>(() => TypeConverter.Mapping<User>("sqlite"));
    }

    [Fact]
    public void InfersAllClrTypesAndSkipsIndexers()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<Mixed>("sqlite");
        Assert.Equal("TEXT", mapping.Schema["label"]);
        Assert.Equal("INTEGER", mapping.Schema["count"]);
        Assert.Equal("REAL", mapping.Schema["ratio"]);
        Assert.Equal("NUMERIC", mapping.Schema["amount"]);
        Assert.Equal("BLOB", mapping.Schema["data"]);
        Assert.Equal("UUID", mapping.Schema["key"]);
        Assert.Equal("INTEGER", mapping.Schema["optional"]);
        Assert.Equal("TEXT", mapping.Schema["extra"]);
        Assert.Equal("INTEGER", mapping.Schema["tiny"]);
        Assert.Equal("REAL", mapping.Schema["real"]);
        Assert.Equal("TEXT", mapping.Schema["xml_parser"]);
        Assert.Equal("INTEGER", mapping.Schema["user2_name"]);
        Assert.Equal("TEXT", mapping.Schema["http"]);
        Assert.False(mapping.Schema.ContainsKey("item"));
        Assert.Equal("INTEGER", TypeConverter.InferSqlType(typeof(sbyte)));
        Assert.Equal("INTEGER", TypeConverter.InferSqlType(typeof(ushort)));
        Assert.Equal("INTEGER", TypeConverter.InferSqlType(typeof(uint)));
        Assert.Equal("INTEGER", TypeConverter.InferSqlType(typeof(ulong)));
        Assert.Equal("TEXT", TypeConverter.InferSqlType(typeof(Uri)));
    }

    [Fact]
    public void SkipsMissingAndWriteOnlyProperties()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<WriteOnly>("sqlite", names: new Dictionary<string, string> { ["id"] = "Id" });
        var obj = mapping.To(new Dictionary<string, object?> { ["id"] = 1, ["missing"] = 2, ["secret"] = 3 });
        Assert.Equal(1, obj.Id);
    }

    [Fact]
    public void BuildsAliasTables()
    {
        var table = TypeConverter.SqlProviderTypes(new Dictionary<string, string> { ["VARCHAR"] = "TEXT", ["NOPE"] = "MISSING" });
        Assert.True(table.Rename);
        Assert.Same(table.Types["TEXT"], table.Types["VARCHAR"]);
        Assert.Same(FieldConverters.Identity, table.Types["NOPE"]);
        Assert.False(TypeConverter.SqlProviderTypes(rename: false).Rename);
        Assert.True((bool)TypeConverter.CanonicalConverters("native")["BOOLEAN"].From(true)!);
        Assert.Equal("VARCHAR", TypeConverter.Normalize(" varchar(32) "));
        Assert.Equal("TINYINT(1)", TypeConverter.Normalize("TINYINT(1)"));
        Assert.Equal("CHAR(36)", TypeConverter.Normalize("CHAR(36)"));
        Assert.Equal("TEXT", TypeConverter.Normalize("TEXT"));
    }

    [Fact]
    public void FieldConvertersCoverBranches()
    {
        Assert.Equal("a", FieldConverters.Identity.To("a"));
        var integer = FieldConverters.Boolean();
        var native = FieldConverters.Boolean("native");
        Assert.True((bool)integer.To(1)!);
        Assert.True((bool)integer.To(1L)!);
        Assert.False((bool)integer.To(0L)!);
        Assert.True((bool)integer.To((byte)1)!);
        Assert.False((bool)integer.To((byte)0)!);
        Assert.True((bool)integer.To((short)1)!);
        Assert.False((bool)integer.To((sbyte)0)!);
        Assert.Throws<DatasourceException>(() => integer.To(2));
        Assert.True((bool)integer.To(true)!);
        Assert.False((bool)integer.To(false)!);
        Assert.Null(integer.To(null));
        Assert.Equal(1, integer.From(true));
        Assert.Equal(0, integer.From(false));
        Assert.Null(integer.From(null));
        Assert.True((bool)native.From(true)!);
        Assert.False((bool)native.From(false)!);
        Assert.Throws<DatasourceException>(() => integer.To("yes"));
        var iso = "2020-01-02T03:04:05.000Z";
        Assert.Equal(2020, ((DateTime)FieldConverters.DateTime.To(iso)!).Year);
        Assert.Equal(2020, ((DateTime)FieldConverters.DateTime.To(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc))!).Year);
        Assert.Null(FieldConverters.DateTime.To(null));
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.From(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc))!);
        Assert.Null(FieldConverters.DateTime.From(null));
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.From(iso)!);
        Assert.Throws<DatasourceException>(() => FieldConverters.DateTime.To(1));
        var plain = FieldConverters.Decimal();
        var sized = FieldConverters.Decimal(5, 2);
        var whole = FieldConverters.Decimal(3, 0);
        Assert.Null(plain.To(null));
        Assert.Equal("12.5", plain.To(12.5));
        Assert.Equal("10", plain.To(10));
        Assert.Equal("10", plain.To(10L));
        Assert.Equal("1.5", plain.To(1.5m));
        Assert.Equal("1.20", sized.To("1.2"));
        Assert.Equal("-3.00", sized.To(-3));
        Assert.Equal("12", whole.To("12"));
        Assert.Throws<DatasourceException>(() => sized.To("1234.5"));
        Assert.Throws<DatasourceException>(() => plain.To(true));
        Assert.Throws<DatasourceException>(() => sized.To("x"));
        var bytes = new byte[] { 104, 105 };
        Assert.Equal("aGk=", FieldConverters.Binary.To(bytes));
        Assert.Equal("aGk=", FieldConverters.Binary.To("aGk="));
        Assert.Null(FieldConverters.Binary.To(null));
        Assert.Equal(bytes, FieldConverters.Binary.From("aGk="));
        Assert.Equal(bytes, FieldConverters.Binary.From(bytes));
        Assert.Null(FieldConverters.Binary.From(null));
        Assert.Throws<DatasourceException>(() => FieldConverters.Binary.To(1));
        Assert.Throws<DatasourceException>(() => FieldConverters.Binary.From(1));
        var id = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        Assert.Equal(id, FieldConverters.Uuid.To(id.ToString()));
        Assert.Equal(id, FieldConverters.Uuid.From(id));
        Assert.Null(FieldConverters.Uuid.To(null));
        Assert.Throws<DatasourceException>(() => FieldConverters.Uuid.To("nope"));
    }
}
