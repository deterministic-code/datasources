using Datasources;

namespace Datasources.Sqlite;

public sealed class BinaryConverter : TypeConverter<Value, Value>
{
    public override Value Convert(Value from) => from;
    public override Value ReverseConvert(Value to) => to;
}
