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

    public static IReadOnlyDictionary<string, T> IdentityFilter<T>(IReadOnlyDictionary<string, T> obj) =>
        obj.ContainsKey("id")
            ? new Dictionary<string, T> { ["id"] = obj["id"] }
            : new Dictionary<string, T>(obj);

    public static string IdentityWhereText<T>(IReadOnlyDictionary<string, T> obj) =>
        obj.ContainsKey("id") ? "id = $id"
        : obj.Count == 0 ? "1 = 1"
        : string.Join(" AND ", obj.Keys.Select(key => $"{key} = ${key}"));
}
