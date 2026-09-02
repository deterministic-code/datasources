namespace Datasources.Postgres;

public sealed class BoolConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Int(var n) => n != 0,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;
}
