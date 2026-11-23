using System.Globalization;
using Datasources;

namespace Datasources.Mysql;

internal sealed class IntegerConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) => Parse(text),
        Value.Float(var n) when double.IsFinite(n) && n == Math.Truncate(n)
            && n is >= long.MinValue and <= long.MaxValue => (long)n,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;

    static Value Parse(string text) =>
        long.TryParse(text, NumberStyles.Integer, CultureInfo.InvariantCulture, out var number)
            ? number
            : decimal.TryParse(text, NumberStyles.Number, CultureInfo.InvariantCulture, out var dec)
              && dec == decimal.Truncate(dec)
              && dec is >= long.MinValue and <= long.MaxValue
                ? (long)dec
                : throw new FormatException($"\"{text}\" is not a long");
}
