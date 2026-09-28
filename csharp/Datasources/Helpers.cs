namespace Datasources;

public static class Helpers
{
    public static Row ParametersToRecord(IReadOnlyList<Parameter>? parameters) =>
        new(ParametersToRecord<Value>(parameters));

    public static IReadOnlyDictionary<string, T> ParametersToRecord<T>(IReadOnlyList<Parameter<T>>? parameters)
    {
        var record = new Dictionary<string, T>();
        if (parameters is null) return record;
        foreach (var parameter in parameters) record[parameter.Name] = parameter.Value;
        return record;
    }
}
