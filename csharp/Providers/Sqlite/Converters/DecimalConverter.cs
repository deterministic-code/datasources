using System.Globalization;
using Datasources;

namespace Datasources.Sqlite;

public sealed class DecimalConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Int(var n) => new Value.Decimal(n),
        Value.Text(var text) when decimal.TryParse(
            text, NumberStyles.Number, CultureInfo.InvariantCulture, out var number) =>
            new Value.Decimal(number),
        _ => from
    };

    public override Value ReverseConvert(Value to) => to switch
    {
        Value.Decimal(var number) => new Value.Text(number.ToString(CultureInfo.InvariantCulture)),
        _ => to
    };
}
