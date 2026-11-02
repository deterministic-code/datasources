using System.Globalization;
using Datasources;

namespace Datasources.Sqlite;

internal sealed class DecimalConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => Canonical(from);

    public override Value ReverseConvert(Value to) => Canonical(to);

    static Value Canonical(Value value) => value switch
    {
        Value.Int(var n) => Text(n),
        Value.Float(var n) when !double.IsNaN(n) && !double.IsInfinity(n) => Text((decimal)n),
        Value.Decimal(var n) => Text(n),
        Value.Text(var text) when decimal.TryParse(
            text, NumberStyles.Number, CultureInfo.InvariantCulture, out var number) =>
            Text(number),
        _ => value
    };

    static Value Text(decimal number) =>
        new Value.Text(number.ToString("0.#############################", CultureInfo.InvariantCulture));
}
