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
        var user = mapping.To(new Dictionary<string, Value>
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
        Assert.Equal(new Value.Int(7), row["id"]);
        Assert.Equal(new Value.Text("ada"), row["username"]);
        Assert.Equal(new Value.Int(1), row["is_enabled"]);
        Assert.Contains("2020-01-02", (string)row["created"].ToObject()!);
    }

    [Fact]
    public void MergesSchemaAndConverters()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var twice = new FieldConverter(
            v => v is int n ? n * 2 : v is long l ? l * 2 : v,
            v => v is int n ? n / 2 : v is long l ? l / 2 : v);
        var mapping = TypeConverter.Mapping<User>("sqlite", schema: new Dictionary<string, string> { ["username"] = "VARCHAR(32)" }, converters: new Dictionary<string, IFieldConverter> { ["id"] = twice }, names: new Dictionary<string, string> { ["id"] = "Id", ["username"] = "Username" });
        Assert.Equal("VARCHAR(32)", mapping.Schema["username"]);
        Assert.Equal(6, mapping.To(new Dictionary<string, Value> { ["id"] = 3, ["username"] = "x" }).Id);
        Assert.Equal(new Value.Int(3), mapping.From(new User { Id = 6, Username = "x" })["id"]);
    }

    [Fact]
    public void MapsDictionaryWithoutInferredSchema()
    {
        TypeConverter.Use("sqlite", TypeConverter.SqlProviderTypes());
        var mapping = TypeConverter.Mapping<Dictionary<string, Value>>("sqlite");
        Assert.Empty(mapping.Schema);
        var dest = mapping.To(new Dictionary<string, Value> { ["display_name"] = "a" });
        Assert.Equal(new Value.Text("a"), dest["DisplayName"]);
        Assert.Equal(new Value.Text("a"), mapping.From(new Dictionary<string, Value> { ["DisplayName"] = "a" })["display_name"]);
        var schemaOnly = TypeConverter.Mapping<Dictionary<string, Value>>(
            "sqlite",
            schema: new Dictionary<string, string> { ["display_name"] = "TEXT", ["___"] = "TEXT", [""] = "TEXT" });
        var converted = schemaOnly.To(new Dictionary<string, Value> { ["display_name"] = "a", ["___"] = "x", [""] = "z" });
        Assert.Equal(new Value.Text("a"), converted["displayName"]);
        TypeConverter.Use("json", TypeConverter.IdentityProviderTypes());
        var json = TypeConverter.Mapping<Dictionary<string, Value>>(
            "json",
            schema: new Dictionary<string, string> { ["keep"] = "TEXT" });
        Assert.Equal(new Value.Text("a"), json.To(new Dictionary<string, Value> { ["keep"] = "a" })["keep"]);
    }

    [Fact]
    public void IdentityProviderKeepsKeys()
    {
        TypeConverter.Use("json", TypeConverter.IdentityProviderTypes());
        var mapping = TypeConverter.Mapping<User>("json");
        Assert.Equal("BOOLEAN", mapping.Schema["IsEnabled"]);
        Assert.True(mapping.To(new Dictionary<string, Value> { ["IsEnabled"] = true }).IsEnabled);
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
        var obj = mapping.To(new Dictionary<string, Value> { ["id"] = 1, ["missing"] = 2, ["secret"] = 3 });
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
        Assert.True((bool)TypeConverter.CanonicalConverters("native")["BOOLEAN"].From(true).ToObject()!);
        Assert.Equal("VARCHAR", TypeConverter.Normalize(" varchar(32) "));
        Assert.Equal("TINYINT(1)", TypeConverter.Normalize("TINYINT(1)"));
        Assert.Equal("CHAR(36)", TypeConverter.Normalize("CHAR(36)"));
        Assert.Equal("TEXT", TypeConverter.Normalize("TEXT"));
    }

    [Fact]
    public void FieldConvertersCoverBranches()
    {
        Assert.Equal(new Value.Text("a"), FieldConverters.Identity.To("a"));
        var integer = FieldConverters.Boolean();
        var native = FieldConverters.Boolean("native");
        Assert.True((bool)integer.To(1).ToObject()!);
        Assert.True((bool)integer.To(1L).ToObject()!);
        Assert.False((bool)integer.To(0L).ToObject()!);
        Assert.True((bool)integer.To(Value.From((byte)1)).ToObject()!);
        Assert.False((bool)integer.To(Value.From((byte)0)).ToObject()!);
        Assert.True((bool)integer.To(Value.From((short)1)).ToObject()!);
        Assert.False((bool)integer.To(Value.From((sbyte)0)).ToObject()!);
        Assert.Throws<DatasourceException>(() => integer.To(2));
        Assert.True((bool)integer.To(true).ToObject()!);
        Assert.False((bool)integer.To(false).ToObject()!);
        Assert.Equal(Value.Null.Instance, integer.To(null));
        Assert.Equal(new Value.Int(1), integer.From(true));
        Assert.Equal(new Value.Int(0), integer.From(false));
        Assert.Equal(Value.Null.Instance, integer.From(null));
        Assert.True((bool)native.From(true).ToObject()!);
        Assert.False((bool)native.From(false).ToObject()!);
        Assert.Throws<DatasourceException>(() => integer.To("yes"));
        var iso = "2020-01-02T03:04:05.000Z";
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.To(iso).ToObject()!);
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.To(Value.From(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc))).ToObject()!);
        Assert.Equal(Value.Null.Instance, FieldConverters.DateTime.To(null));
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.From(Value.From(new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc))).ToObject()!);
        Assert.Equal(Value.Null.Instance, FieldConverters.DateTime.From(null));
        Assert.Contains("2020-01-02", (string)FieldConverters.DateTime.From(iso).ToObject()!);
        Assert.Throws<DatasourceException>(() => FieldConverters.DateTime.To(1));
        var plain = FieldConverters.Decimal();
        var sized = FieldConverters.Decimal(5, 2);
        var whole = FieldConverters.Decimal(3, 0);
        Assert.Equal(Value.Null.Instance, plain.To(null));
        Assert.Equal(new Value.Text("12.5"), plain.To(12.5));
        Assert.Equal(new Value.Text("10"), plain.To(10));
        Assert.Equal(new Value.Text("10"), plain.To(10L));
        Assert.Equal(new Value.Text("1.5"), plain.To(Value.From(1.5m)));
        Assert.Equal(new Value.Text("1.20"), sized.To("1.2"));
        Assert.Equal(new Value.Text("-3.00"), sized.To(-3));
        Assert.Equal(new Value.Text("12"), whole.To("12"));
        Assert.Throws<DatasourceException>(() => sized.To("1234.5"));
        Assert.Throws<DatasourceException>(() => plain.To(true));
        Assert.Throws<DatasourceException>(() => sized.To("x"));
        var bytes = new byte[] { 104, 105 };
        Assert.Equal(new Value.Text("aGk="), FieldConverters.Binary.To(Value.From(bytes)));
        Assert.Equal(new Value.Text("aGk="), FieldConverters.Binary.To("aGk="));
        Assert.Equal(Value.Null.Instance, FieldConverters.Binary.To(null));
        Assert.Equal(new Value.Text("aGk="), FieldConverters.Binary.From("aGk="));
        Assert.Equal(new Value.Text("aGk="), FieldConverters.Binary.From(Value.From(bytes)));
        Assert.Equal(Value.Null.Instance, FieldConverters.Binary.From(null));
        Assert.Throws<DatasourceException>(() => FieldConverters.Binary.To(1));
        Assert.Throws<DatasourceException>(() => FieldConverters.Binary.From(1));
        var id = Guid.Parse("550e8400-e29b-41d4-a716-446655440000");
        Assert.Equal(new Value.Text(id.ToString()), FieldConverters.Uuid.To(id.ToString()));
        Assert.Equal(new Value.Text(id.ToString()), FieldConverters.Uuid.From(Value.From(id)));
        Assert.Equal(Value.Null.Instance, FieldConverters.Uuid.To(null));
        Assert.Throws<DatasourceException>(() => FieldConverters.Uuid.To("nope"));
    }
}
