namespace Datasources;

public record Parameter<T>(
    string Name,
    T Value,
    ParameterDirection Direction = ParameterDirection.Input);

public sealed record Parameter(
    string Name,
    Value Value,
    ParameterDirection Direction = ParameterDirection.Input)
    : Parameter<Value>(Name, Value, Direction)
{
    public Parameter(string name, int value) : this(name, (Value)value) { }
    public Parameter(string name, long value) : this(name, (Value)value) { }
    public Parameter(string name, bool value) : this(name, (Value)value) { }

    public Parameter(string name, ParameterDirection direction)
        : this(name, Value.Null.Instance, direction) { }
}
