using Datasources;

namespace Datasources.Sqlite;

internal sealed class BoolConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Int(var n) => n != 0,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to switch
    {
        Value.Bool(var flag) => flag ? 1L : 0L,
        _ => to
    };
}
