using Datasources;

namespace Datasources.Sqlite;

internal sealed class IntegerConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from switch
    {
        Value.Text(var text) => long.TryParse(text, out var number)
            ? number
            : throw new FormatException($"\"{text}\" is not a long"),
        _ => from
    };

    public override Value ReverseConvert(Value to) => to;
}
