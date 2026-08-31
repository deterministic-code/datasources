using Datasources;

namespace Datasources.Tests;

public class ConverterTests
{
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
        public int this[int i] { get => 0; set { } }
    }

    public class WriteOnly
    {
        public int Id { get; set; }
        public int Secret { set { } }
    }

    [Fact]
    public void ConvertAndReverseConvertRoundTrip()
    {
        var converter = new RowConverter<User>();
        var user = converter.Convert(new Row
        {
            ["Id"] = 7,
            ["Username"] = "ada",
            ["IsEnabled"] = 1,
            ["Created"] = "2020-01-02T03:04:05.000Z"
        });
        Assert.Equal(7, user.Id);
        Assert.Equal("ada", user.Username);
        Assert.True(user.IsEnabled);
        Assert.Equal(2020, user.Created.Year);
        var row = converter.ReverseConvert(user);
        Assert.Equal(new Value.Int(7), row["Id"]);
        Assert.Equal(new Value.Text("ada"), row["Username"]);
    }

    [Fact]
    public void MapsDictionaryRows()
    {
        var converter = new RowConverter<Dictionary<string, Value>>();
        var dest = converter.Convert(new Row { ["display_name"] = "a" });
        Assert.Equal(new Value.Text("a"), dest["display_name"]);
        Assert.Equal(new Value.Text("a"), converter.ReverseConvert(dest)["display_name"]);
    }

    [Fact]
    public void SkipsIndexersAndWriteOnlyProperties()
    {
        var mixed = new RowConverter<Mixed>().Convert(new Row { ["Label"] = "x", ["Count"] = 1L, ["Item"] = 2 });
        Assert.Equal("x", mixed.Label);
        Assert.Equal(1, mixed.Count);
        var obj = new RowConverter<WriteOnly>().Convert(new Row { ["Id"] = 1, ["missing"] = 2, ["Secret"] = 3 });
        Assert.Equal(1, obj.Id);
    }

    sealed class OnlyOne : TypeConverter<object?, Value>
    {
        public override object? Convert(Value from) => from.ToObject() is long n && n == 1;
        public override Value ReverseConvert(object? to) => Value.From(to);
    }

    [Fact]
    public void FieldOverrideBeatsTypeDefault()
    {
        var row = new Row
        {
            ["Id"] = 1,
            ["Username"] = "a",
            ["IsEnabled"] = 2,
            ["Created"] = "2020-01-02T00:00:00.000Z"
        };
        Assert.True(new RowConverter<User>().Convert(row).IsEnabled);
        var converter = new RowConverter<User>(null, new Dictionary<string, TypeConverter<object?, Value>>
        {
            ["IsEnabled"] = new OnlyOne()
        });
        Assert.False(converter.Convert(row).IsEnabled);
    }

    [Fact]
    public void AppliesFieldOverrideOnDictionaryRows()
    {
        var converter = new RowConverter<Dictionary<string, Value>>(null, new Dictionary<string, TypeConverter<object?, Value>>
        {
            ["flag"] = new OnlyOne()
        });
        var dest = converter.Convert(new Row { ["flag"] = 1L, ["name"] = "a" });
        Assert.Equal(new Value.Bool(true), dest["flag"]);
        Assert.Equal(new Value.Text("a"), dest["name"]);
        Assert.Equal(new Value.Bool(true), converter.ReverseConvert(dest)["flag"]);
    }

    [Fact]
    public void RegexAndReplace()
    {
        Assert.Equal("Ada  Lovelace", new RegexConverter(@"\d+", "").Convert("Ada 1 Lovelace"));
        Assert.Equal("Ada  Lovelace", new RegexConverter(new System.Text.RegularExpressions.Regex(@"\d+"), "").Convert("Ada 1 Lovelace"));
        Assert.Equal("id-7", new RegexConverter(@"id_(\d+)", "id-$1").Convert("id_7"));
        Assert.Equal("id_7", new RegexConverter(@"id_(\d+)", "id-$1").ReverseConvert("id_7"));
        Assert.Equal("Ada Lovelace", new ReplaceConverter("_", " ").Convert("Ada_Lovelace"));
        Assert.Equal("Ada_Lovelace", new ReplaceConverter("_", " ").ReverseConvert("Ada Lovelace"));
    }

    [Fact]
    public void ToLowerAndToUpperCase()
    {
        Assert.Equal("ada", new ToLowerCaseConverter().Convert("ADA"));
        Assert.Equal("ADA", new ToLowerCaseConverter().ReverseConvert("ADA"));
        Assert.Equal("ADA", new ToUpperCaseConverter().Convert("ada"));
        Assert.Equal("ada", new ToUpperCaseConverter().ReverseConvert("ada"));
    }

    static readonly (Casing Casing, string Text)[] Forms =
    [
        (Casing.Camel, "twoWords"),
        (Casing.Capital, "Two Words"),
        (Casing.Constant, "TWO_WORDS"),
        (Casing.Dot, "two.words"),
        (Casing.Kebab, "two-words"),
        (Casing.No, "two words"),
        (Casing.Pascal, "TwoWords"),
        (Casing.PascalSnake, "Two_Words"),
        (Casing.Path, "two/words"),
        (Casing.Sentence, "Two words"),
        (Casing.Snake, "two_words"),
        (Casing.Train, "Two-Words"),
    ];

    [Fact]
    public void ChangeCasing()
    {
        foreach (var (_, from) in Forms)
        {
            foreach (var (casing, expected) in Forms)
                Assert.Equal(expected, new ChangeCasingConverter(casing).Convert(from));
            Assert.Equal(from, new ChangeCasingConverter(Casing.Snake).ReverseConvert(from));
        }
        Assert.Equal("xmlParser", new ChangeCasingConverter(Casing.Camel).Convert("XMLParser"));
        Assert.Equal("html5", new ChangeCasingConverter(Casing.Camel).Convert("html5"));
        Assert.Equal("v2Name", new ChangeCasingConverter(Casing.Camel).Convert("v2Name"));
        Assert.Equal("", new ChangeCasingConverter(Casing.Snake).Convert(""));
        Assert.Equal("", new ChangeCasingConverter(Casing.Snake).Convert("___"));
        Assert.Equal("two_words", new ChangeCasingConverter(Casing.Snake).Convert(" two  words\t"));
        Assert.Equal("", new ChangeCasingConverter(Casing.Camel).Convert(""));
        Assert.Throws<ArgumentOutOfRangeException>(() => new ChangeCasingConverter((Casing)99).Convert("a"));
    }

    [Fact]
    public void PluralizesAndSingularizes()
    {
        var pairs = new (string Singular, string Plural)[]
        {
            ("person", "people"),
            ("child", "children"),
            ("man", "men"),
            ("woman", "women"),
            ("mouse", "mice"),
            ("goose", "geese"),
            ("tooth", "teeth"),
            ("foot", "feet"),
            ("ox", "oxen"),
            ("sheep", "sheep"),
            ("fish", "fish"),
            ("series", "series"),
            ("species", "species"),
            ("deer", "deer"),
            ("news", "news"),
            ("box", "boxes"),
            ("dish", "dishes"),
            ("church", "churches"),
            ("quiz", "quizes"),
            ("kiss", "kisses"),
            ("bus", "buses"),
            ("city", "cities"),
            ("cat", "cats"),
            ("toy", "toys"),
            ("2y", "2ys"),
            ("Person", "People"),
            ("", ""),
        };
        var converter = new PluralizerConverter();
        foreach (var (singular, plural) in pairs)
        {
            Assert.Equal(plural, converter.Convert(singular));
            Assert.Equal(plural, converter.Convert(plural));
            Assert.Equal(singular, converter.ReverseConvert(plural));
            Assert.Equal(singular, converter.ReverseConvert(singular));
        }
    }
}
