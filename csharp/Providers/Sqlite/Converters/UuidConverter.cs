using Datasources;

namespace Datasources.Sqlite;

internal sealed class UuidConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => Canonical(from);

    public override Value ReverseConvert(Value to) => Canonical(to);

    static Value Canonical(Value value) => value switch
    {
        Value.Guid(var guid) => Text(guid),
        Value.Text(var text) when System.Guid.TryParse(text, out var guid) => Text(guid),
        Value.Binary(var bytes) when bytes.Length == 16 => Text(new System.Guid(bytes)),
        _ => value
    };

    static Value Text(System.Guid guid) => new Value.Text(guid.ToString("D"));
}
