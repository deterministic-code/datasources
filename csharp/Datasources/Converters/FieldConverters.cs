using System.Globalization;
using System.Text.RegularExpressions;

namespace Datasources;

public static class FieldConverters
{
    public static readonly IFieldConverter Identity = new FieldConverter(v => v, v => v);

    public static IFieldConverter Boolean(string storage = "integer") => new FieldConverter(
        v => v is null ? null : AsBool(v),
        v => v is null ? null : storage == "native" ? AsBool(v) : AsBool(v) ? 1 : 0);

    public static readonly IFieldConverter DateTime = new FieldConverter(
        v => v is null ? null : AsDate(v),
        v => v is null ? null : AsDate(v).ToUniversalTime().ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'"));

    public static IFieldConverter Decimal(int? precision = null, int? scale = null) => new FieldConverter(
        v => FormatDecimal(v, precision, scale),
        v => FormatDecimal(v, precision, scale));

    public static readonly IFieldConverter Binary = new FieldConverter(
        v => v is null ? null : v is string text ? text : v is byte[] bytes ? Convert.ToBase64String(bytes) : throw Fail(v, "binary"),
        v => v is null ? null : v is byte[] bytes ? bytes : v is string text ? Convert.FromBase64String(text) : throw Fail(v, "bytes"));

    public static readonly IFieldConverter Uuid = new FieldConverter(ParseUuid, ParseUuid);

    static bool AsBool(object value)
    {
        if (value is bool flag) return flag;
        if (value is byte or sbyte or short or int or long)
            return Convert.ToInt64(value) switch
            {
                0 => false,
                1 => true,
                _ => throw Fail(value, "boolean")
            };
        throw Fail(value, "boolean");
    }

    static DateTime AsDate(object value) =>
        value is DateTime date ? date
        : value is string text && System.DateTime.TryParse(text, CultureInfo.InvariantCulture, DateTimeStyles.RoundtripKind, out var parsed)
            ? parsed
            : throw Fail(value, "DateTime");

    static object? FormatDecimal(object? value, int? precision, int? scale)
    {
        if (value is null) return null;
        var text = value switch
        {
            string s => s,
            decimal d => d.ToString(CultureInfo.InvariantCulture),
            int n => n.ToString(CultureInfo.InvariantCulture),
            long n => n.ToString(CultureInfo.InvariantCulture),
            double n => n.ToString(CultureInfo.InvariantCulture),
            _ => throw Fail(value, "decimal")
        };
        if (precision is null || scale is null) return text;
        var match = DecimalPattern().Match(text);
        if (!match.Success) throw Fail(value, "decimal");
        var sign = match.Groups[1].Value;
        var whole = match.Groups[2].Value;
        var frac = match.Groups[3].Value.PadRight(scale.Value, '0')[..scale.Value];
        if (whole.Length + (scale > 0 ? frac.Length : 0) > precision)
            throw new DatasourceException($"Decimal {text} exceeds size [{precision}, {scale}]");
        return scale > 0 ? $"{sign}{whole}.{frac}" : $"{sign}{whole}";
    }

    static object? ParseUuid(object? value) =>
        value is null ? null
        : value is Guid guid ? guid
        : value is string text && Guid.TryParse(text, out var parsed) ? parsed
        : throw Fail(value, "UUID");

    static DatasourceException Fail(object value, string target) =>
        new($"Cannot convert {value} to {target}");

    static Regex DecimalPattern() => new(@"^(-?)(\d+)(?:\.(\d+))?$");
}
