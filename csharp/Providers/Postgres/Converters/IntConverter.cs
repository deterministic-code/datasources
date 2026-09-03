using System.Globalization;

namespace Datasources.Postgres;

public sealed class IntConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) when long.TryParse(
            text, NumberStyles.Integer, CultureInfo.InvariantCulture, out var number) =>
            new Value.Int(number),
        _ => from
    };

    public override Value ReverseConvert(Value to) => Convert(to);
}
