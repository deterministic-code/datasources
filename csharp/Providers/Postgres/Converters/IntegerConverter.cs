using Datasources;

namespace Datasources.Postgres;

internal sealed class IntegerConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) when text.Length > 0 && long.TryParse(text, out var number) => number,
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;
}
