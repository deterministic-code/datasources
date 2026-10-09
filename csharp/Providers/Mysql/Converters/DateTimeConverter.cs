using System.Globalization;

namespace Datasources.Mysql;

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
        Value.DateTime => new Value.Text(Stamp(to.IsoDateTime())),
        Value.Text(var text) when text.Contains('T') && DateTime.TryParse(
            text, CultureInfo.InvariantCulture,
            DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal, out _) =>
            new Value.Text(Stamp(text)),
        _ => to
    };

    static string Stamp(string iso) =>
        DateTime.Parse(iso, CultureInfo.InvariantCulture, DateTimeStyles.AdjustToUniversal | DateTimeStyles.AssumeUniversal)
            .ToUniversalTime()
            .ToString("yyyy-MM-dd HH:mm:ss.fff", CultureInfo.InvariantCulture);
}
