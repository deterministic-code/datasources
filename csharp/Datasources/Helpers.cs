namespace Datasources;

public static class Helpers
{
    public static IReadOnlyDictionary<string, object?> ParametersToRecord(IReadOnlyList<Parameter>? parameters)
    {
        var record = new Dictionary<string, object?>();
        if (parameters is null)
        {
            return record;
        }

        foreach (var parameter in parameters)
        {
            record[parameter.Name] = parameter.Value;
        }

        return record;
    }

    public static IReadOnlyDictionary<string, object?> IdentityFilter(IReadOnlyDictionary<string, object?> obj)
    {
        if (obj.ContainsKey("id"))
        {
            return new Dictionary<string, object?> { ["id"] = obj["id"] };
        }

        return new Dictionary<string, object?>(obj);
    }

    public static string IdentityWhereText(IReadOnlyDictionary<string, object?> obj)
    {
        if (obj.ContainsKey("id"))
        {
            return "id = $id";
        }

        if (obj.Count == 0)
        {
            return "1 = 1";
        }

        return string.Join(" AND ", obj.Keys.Select(key => $"{key} = ${key}"));
    }
}
