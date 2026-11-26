using System.Globalization;
using System.Text;
using YamlDotNet.Core;
using YamlDotNet.RepresentationModel;

namespace Datasources;

public static class ConversionMatrix
{
    public static IReadOnlyList<Dictionary<string, object?>> Rows(string driver, string table)
    {
        var root = Root();
        var driverNode = Map(root, driver);
        var language = Map(driverNode, "csharp");
        return List(language.Children[new YamlScalarNode(table)]);
    }

    public static void Types(string driver, Func<string, string> names, Func<SpecField, string> fields, Func<string, string> reverse)
    {
        foreach (var row in Rows(driver, "typeConverter"))
        {
            var from = Decode(row["from"]);
            Func<object?> act = Reverse(row)
                ? () => reverse(Text(from))
                : () => from is SpecField field ? fields(field) : names(Text(from));
            Conversion(row).Assert(act);
        }
    }

    public static void Defaults(string driver, IDefaultValueConverter defaults)
    {
        foreach (var row in Rows(driver, "defaultValueConverter"))
        {
            var from = Text(Decode(row["from"]));
            Conversion(row).Assert(() =>
                Reverse(row) ? defaults.ReverseConvert(TypeOf(row), from) : defaults.Convert(TypeOf(row), from));
        }
    }

    public static void Cells(string driver, IReadOnlyDictionary<string, TypeConverter<Value, Value>> converters)
    {
        foreach (var row in Rows(driver, "rowTypeConverters"))
        {
            var from = Decode(row["from"]);
            AssertName(row["fromType"], from);
            var converter = converters[TypeOf(row)];
            Conversion(row).Assert(() =>
            {
                var got = (Reverse(row) ? converter.ReverseConvert(Datasources.Value.From(from)) : converter.Convert(Datasources.Value.From(from)))
                    .ToObject();
                if (row["to"] is not Dictionary<string, object?> { } to || !to.ContainsKey("error"))
                    AssertName(row["toType"], got);
                return got;
            });
        }
    }

    static Conversion Conversion(Dictionary<string, object?> row) =>
        new(TypeOf(row), Decode(row["from"]), Decode(row["to"]));

    static bool Reverse(Dictionary<string, object?> row) =>
        row.TryGetValue("reverse", out var reverse) && reverse is true;

    static string TypeOf(Dictionary<string, object?> row) =>
        row["type"] as string ?? throw new InvalidOperationException("type");

    static string Text(object? value) =>
        value as string ?? throw new ArgumentException(value?.GetType().FullName);

    static void AssertName(object? expected, object? value)
    {
        var name = expected as string ?? throw new InvalidOperationException("type name");
        if (name is "FormatException" or "InvalidOperationException" or "Error" or "DatasourceError" or "error")
            return;
        var actual = value switch
        {
            long => "long",
            bool => "bool",
            string => "string",
            double => "double",
            decimal => "decimal",
            DateTime => "DateTime",
            Guid => "Guid",
            byte[] => "byte[]",
            _ => value?.GetType().Name
        };
        Xunit.Assert.Equal(name, actual);
    }

    static object? Decode(object? raw) => raw switch
    {
        Dictionary<string, object?> map when map.TryGetValue("field", out var field) => Spec(field),
        Dictionary<string, object?> map when map.TryGetValue("error", out var error) => ExceptionType(Text(error)),
        Dictionary<string, object?> map when map.TryGetValue("long", out var number) => Convert.ToInt64(number, CultureInfo.InvariantCulture),
        Dictionary<string, object?> map when map.TryGetValue("text", out var text) => Text(text),
        Dictionary<string, object?> map when map.TryGetValue("bool", out var flag) => Convert.ToBoolean(flag, CultureInfo.InvariantCulture),
        Dictionary<string, object?> map when map.TryGetValue("float", out var number) => Convert.ToDouble(number, CultureInfo.InvariantCulture),
        Dictionary<string, object?> map when map.TryGetValue("decimal", out var number) => Convert.ToDecimal(Text(number), CultureInfo.InvariantCulture),
        Dictionary<string, object?> map when map.TryGetValue("datetime", out var text) =>
            DateTime.SpecifyKind(
                DateTime.Parse(Text(text), CultureInfo.InvariantCulture, DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal),
                DateTimeKind.Utc),
        Dictionary<string, object?> map when map.TryGetValue("guid", out var text) => Guid.Parse(Text(text)),
        Dictionary<string, object?> map when map.TryGetValue("bytes", out var bytes) => Bytes(bytes),
        Dictionary<string, object?> map when map.TryGetValue("base64", out var text) => Text(text),
        Dictionary<string, object?> map when map.TryGetValue("latin1", out var bytes) => Encoding.Latin1.GetString(Bytes(bytes)),
        _ => raw
    };

    static SpecField Spec(object? raw)
    {
        var map = raw as Dictionary<string, object?> ?? throw new ArgumentException("field");
        return new SpecField
        {
            Type = Text(map["type"]),
            Name = map.TryGetValue("name", out var name) ? name as string : null,
            Size = map.TryGetValue("size", out var size) ? Size(size) : null
        };
    }

    static object? Size(object? raw) => raw switch
    {
        string text => text,
        int number => number,
        long number => (int)number,
        IEnumerable<object?> items => items.Select(item => Convert.ToInt32(item, CultureInfo.InvariantCulture)).ToArray(),
        _ => raw
    };

    static byte[] Bytes(object? raw) =>
        raw is IEnumerable<object?> items
            ? items.Select(item => Convert.ToByte(item, CultureInfo.InvariantCulture)).ToArray()
            : throw new ArgumentException(raw?.GetType().FullName);

    static Type ExceptionType(string name) => name switch
    {
        "InvalidOperationException" => typeof(InvalidOperationException),
        "FormatException" => typeof(FormatException),
        _ => throw new InvalidOperationException(name)
    };

    static YamlMappingNode Root()
    {
        var yaml = new YamlStream();
        yaml.Load(new StringReader(File.ReadAllText(Path())));
        return (YamlMappingNode)yaml.Documents[0].RootNode;
    }

    static string Path()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var path = System.IO.Path.Combine(dir.FullName, "docs", "DriverConversions.yaml");
            if (File.Exists(path)) return path;
        }
        throw new FileNotFoundException("docs/DriverConversions.yaml");
    }

    static YamlMappingNode Map(YamlMappingNode node, string key) =>
        (YamlMappingNode)node.Children[new YamlScalarNode(key)];

    static List<Dictionary<string, object?>> List(YamlNode node)
    {
        var list = (YamlSequenceNode)node;
        return list.Children.Select(child => Entry((YamlMappingNode)child)).ToList();
    }

    static Dictionary<string, object?> Entry(YamlMappingNode node) =>
        node.Children.ToDictionary(kv => ((YamlScalarNode)kv.Key).Value!, kv => YamlValue(kv.Value));

    static object? YamlValue(YamlNode node) => node switch
    {
        YamlScalarNode scalar => Scalar(scalar),
        YamlSequenceNode sequence => sequence.Children.Select(YamlValue).ToList(),
        YamlMappingNode mapping => mapping.Children.ToDictionary(
            kv => ((YamlScalarNode)kv.Key).Value!,
            kv => YamlValue(kv.Value)),
        _ => throw new InvalidOperationException(node.ToString())
    };

    static object? Scalar(YamlScalarNode scalar)
    {
        if (scalar.IsNull()) return null;
        var text = scalar.Value ?? "";
        if (scalar.Style is ScalarStyle.SingleQuoted or ScalarStyle.DoubleQuoted or ScalarStyle.Literal or ScalarStyle.Folded)
            return text;
        if (text is "true") return true;
        if (text is "false") return false;
        if (text is "null") return null;
        if (int.TryParse(text, NumberStyles.Integer, CultureInfo.InvariantCulture, out var number)) return number;
        if (double.TryParse(text, NumberStyles.Float, CultureInfo.InvariantCulture, out var real)) return real;
        return text;
    }

    static bool IsNull(this YamlScalarNode scalar) =>
        scalar.Tag.ToString() is "tag:yaml.org,2002:null" || scalar.Value is null;
}
