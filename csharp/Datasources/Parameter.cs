namespace Datasources;

public record Parameter<T>(string Name, T Value);

public sealed record Parameter(string Name, Value Value) : Parameter<Value>(Name, Value);
