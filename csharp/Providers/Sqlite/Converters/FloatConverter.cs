using System.Globalization;
using Datasources;

namespace Datasources.Sqlite;

internal sealed class FloatConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Int(var n) => (double)n,
        Value.Float(var n) => n,
        Value.Decimal(var n) => (double)n,
        Value.Text(var text) when double.TryParse(text, NumberStyles.Float, CultureInfo.InvariantCulture, out var number)
            && double.IsFinite(number) => number,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;
}
