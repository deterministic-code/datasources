using System.Text.RegularExpressions;

namespace Datasources.CodeGenerators;

public sealed class DatasourceNaming
{
    static readonly Regex SnakeStem = new(@"^[a-z_][a-z0-9_]*$", RegexOptions.Compiled);
    readonly CasingStrategy casing;
    readonly bool pluralizeNames;

    DatasourceNaming(CasingStrategy casing, bool pluralizeNames) =>
        (this.casing, this.pluralizeNames) = (casing, pluralizeNames);

    public static DatasourceNaming Of(IReadOnlyDictionary<string, string> settings) =>
        new(
            CasingStrategy.Sql(settings, "datasource.casing"),
            !settings.TryGetValue("datasource.pluralize_datatable_names", out var flag) || flag != "false");

    public bool IsVerbatim(string mapping) => !SnakeStem.IsMatch(mapping);

    public string PhysicalStem(string name) =>
        IsVerbatim(name) ? name : pluralizeNames ? LastTokenPluralize.Of(name) : name;

    public string TableName(string stem) => casing.ConvertTypes(stem);

    public string ColumnName(string stem) => casing.ConvertFields(stem);

    public string ResolveTable(string logical, string? mapping = null) =>
        Resolve(stem => TableName(PhysicalStem(stem)), logical, mapping);

    public string ResolveColumn(string logical, string? mapping = null) =>
        Resolve(ColumnName, logical, mapping);

    string Resolve(Func<string, string> convert, string logical, string? mapping)
    {
        var stem = mapping ?? logical;
        return IsVerbatim(stem) ? stem : convert(stem);
    }
}
