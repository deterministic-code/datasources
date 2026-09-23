using System.Globalization;

namespace Datasources.Postgres;

internal sealed class DateTimeConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) when DateTime.TryParse(
            text, CultureInfo.InvariantCulture,
            DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal, out var date) =>
            new Value.DateTime(DateTime.SpecifyKind(date, DateTimeKind.Utc)),
        _ => from
    };

    public override Value ReverseConvert(Value to) => to switch
    {
        Value.DateTime => new Value.Text(to.IsoDateTime()),
        _ => to
    };
}
