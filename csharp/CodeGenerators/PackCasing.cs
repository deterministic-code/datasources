namespace Datasources.CodeGenerators;

public sealed class PackCasing
{
    const string Prefix = "datasource.casing";
    readonly DatasourceNaming naming;
    readonly DatasourceNaming alwaysPlural;

    PackCasing(
        DatasourceNaming naming,
        DatasourceNaming alwaysPlural,
        StatementNaming strategy)
    {
        this.naming = naming;
        this.alwaysPlural = alwaysPlural;
        Naming = strategy;
    }

    public StatementNaming Naming { get; }

    public static PackCasing Of(IReadOnlyDictionary<string, string> settings)
    {
        var always = new Dictionary<string, string>(settings) { ["datasource.pluralize_datatable_names"] = "true" };
        var objects = ParseUpperLower(Get(settings, $"{Prefix}.objects"), $"{Prefix}.objects", "preserve");
        var keywords = ParseUpperLower(Get(settings, $"{Prefix}.keywords"), $"{Prefix}.keywords", "upper");
        var types = Identifier(settings, $"{Prefix}.types");
        return new(
            DatasourceNaming.Of(settings),
            DatasourceNaming.Of(always),
            new StatementNaming
            {
                TableNameCasing = types,
                FieldNameCasing = Identifier(settings, $"{Prefix}.fields"),
                ConstraintNameCasing = types,
                TriggerNameCasing = types,
                KeywordCasing = keywords == "lower" ? KeywordCasing.Lower : KeywordCasing.Upper,
                PluralizeTables = !settings.TryGetValue("datasource.pluralize_datatable_names", out var flag) || flag != "false",
                Objects = objects,
            });
    }

    public bool IsVerbatim(string mapping) => naming.IsVerbatim(mapping);
    public string PluralTableName(string entity) => ObjectIdent(entity, alwaysPlural.ResolveTable(entity));
    public string PluralStem(string entity) => alwaysPlural.PhysicalStem(entity);
    public string RoutineName(string stem) => ObjectIdent(stem, naming.TableName(stem));

    string ObjectIdent(string stem, string converted) =>
        naming.IsVerbatim(stem) ? converted : Naming.Letter(converted);

    static Casing Identifier(IReadOnlyDictionary<string, string> settings, string path)
    {
        var raw = Get(settings, path);
        if (string.IsNullOrEmpty(raw) || raw.Equals("auto", StringComparison.OrdinalIgnoreCase))
            return Casing.Snake;
        return raw.ToLowerInvariant() switch
        {
            "camel" => Casing.Camel,
            "pascal" => Casing.Pascal,
            "snake" => Casing.Snake,
            "kebab" => Casing.Kebab,
            _ => throw new InvalidOperationException($"{path} must be one of [Camel, Pascal, Snake, Kebab] (got \"{raw}\")."),
        };
    }

    static string ParseUpperLower(string? raw, string path, string fallback)
    {
        if (string.IsNullOrEmpty(raw)) return fallback;
        var parsed = raw.ToLowerInvariant();
        if (parsed == "auto") return fallback;
        return parsed is "upper" or "lower"
            ? parsed
            : throw new InvalidOperationException($"{path} must be one of [upper, lower] (got \"{raw}\").");
    }

    static string? Get(IReadOnlyDictionary<string, string> settings, string key) =>
        settings.TryGetValue(key, out var value) ? value : null;
}
