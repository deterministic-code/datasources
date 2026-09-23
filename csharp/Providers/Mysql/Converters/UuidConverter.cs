namespace Datasources.Mysql;

internal sealed class UuidConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) when Guid.TryParse(text, out var guid) => new Value.Guid(guid),
        _ => from
    };

    public override Value ReverseConvert(Value to) => to switch
    {
        Value.Guid(var guid) => new Value.Text(guid.ToString()),
        _ => to
    };
}
