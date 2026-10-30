using System.Reflection;

namespace Datasources.CodeGenerators;

public sealed class PackCasing
{
    const string Prefix = "datasource.casing";
    readonly DatasourceNaming naming;
    readonly DatasourceNaming alwaysPlural;
    readonly CasingStrategy casing;
    readonly string keywordFormat;
    readonly string objectFormat;

    PackCasing(
        DatasourceNaming naming,
        DatasourceNaming alwaysPlural,
        CasingStrategy casing,
        string keywordFormat,
        string objectFormat)
    {
        this.naming = naming;
        this.alwaysPlural = alwaysPlural;
        this.casing = casing;
        this.keywordFormat = keywordFormat;
        this.objectFormat = objectFormat;
    }

    public static PackCasing Of(IReadOnlyDictionary<string, string> settings)
    {
        var always = new Dictionary<string, string>(settings) { ["datasource.pluralize_datatable_names"] = "true" };
        return new(
            DatasourceNaming.Of(settings),
            DatasourceNaming.Of(always),
            CasingStrategy.Sql(settings, Prefix),
            ParseUpperLower(Get(settings, $"{Prefix}.keywords"), $"{Prefix}.keywords", "upper"),
            ParseUpperLower(Get(settings, $"{Prefix}.objects"), $"{Prefix}.objects", "preserve"));
    }

    public bool IsVerbatim(string mapping) => naming.IsVerbatim(mapping);
    public string TableName(string entity) => ObjectIdent(entity, naming.ResolveTable(entity));
    public string PluralTableName(string entity) => ObjectIdent(entity, alwaysPlural.ResolveTable(entity));
    public string PluralStem(string entity) => alwaysPlural.PhysicalStem(entity);
    public string ColumnName(string field) => naming.ResolveColumn(field);
    public string Keyword(string text) => Letter(text, keywordFormat);
    public string ApplyKeywords(string sql, string dialect) =>
        keywordFormat == "lower" ? DriverApply(dialect, sql) : sql;

    string DriverApply(string dialect, string sql)
    {
        var method = GenerateSql.Driver(dialect).GetType()
            .GetMethod("ApplyKeywords", BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static)
            ?? throw new InvalidOperationException($"\"{dialect}\" has no ApplyKeywords.");
        return (string)method.Invoke(null, [sql, (Func<string, string>)Keyword])!;
    }

    public string ConstraintName(string entity, params string[] parts) =>
        naming.IsVerbatim(entity)
            ? string.Join("_", new[] { entity }.Concat(parts))
            : ObjectName(naming.TableName(string.Join("_", new[] { naming.PhysicalStem(entity) }.Concat(parts))));

    public string TriggerName(string entity)
    {
        var stem = naming.PhysicalStem(entity);
        return naming.IsVerbatim(entity)
            ? $"trg_{entity}_updated_at"
            : ObjectName(naming.TableName($"trg_{stem}_updated_at"));
    }

    public string RoutineName(string stem) => ObjectIdent(stem, naming.TableName(stem));

    string ObjectIdent(string stem, string converted) => naming.IsVerbatim(stem) ? converted : ObjectName(converted);

    string ObjectName(string text) => Letter(text, objectFormat);

    static string Letter(string text, string format) => format switch
    {
        "upper" => text.ToUpperInvariant(),
        "lower" => text.ToLowerInvariant(),
        _ => text,
    };

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
