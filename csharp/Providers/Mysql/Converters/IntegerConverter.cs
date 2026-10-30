using Datasources;

namespace Datasources.Mysql;

internal sealed class IntegerConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text("") => 0L,
        Value.Text(var text) when long.TryParse(text, out var number) => number,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;
}
