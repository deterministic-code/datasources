using System.Collections.Concurrent;
using System.Reflection;

namespace Datasources.CodeGenerators;

public static class SqlDialect
{
    static readonly ConcurrentDictionary<string, SqlGenerateDriver> Loaded = new();

    public static void Register(string dialect, IDriver driver) =>
        Loaded[RequireName(dialect)] = From(dialect, driver);

    public static SqlGenerateDriver DriverOf(string dialect) =>
        Loaded.TryGetValue(RequireName(dialect), out var driver)
            ? driver
            : throw new ProviderNotInstalledException(dialect);

    static SqlGenerateDriver From(string dialect, IDriver driver) =>
        driver.TypeConverter is DeterministicTypeConverter types
        && driver.DefaultValueConverter is { } defaults
        && driver.StatementRenderer is { } renderer
        && Quote(driver, "QuoteLeft") is { } left
        && Quote(driver, "QuoteRight") is { } right
            ? new SqlGenerateDriver
            {
                Driver = driver,
                QuoteLeft = left,
                QuoteRight = right,
                TypeConverter = types,
                DefaultValueConverter = defaults,
                StatementRenderer = renderer,
            }
            : throw new InvalidOperationException($"\"{dialect}\" is not a SQL dialect.");

    static string RequireName(string dialect) =>
        ProviderCatalog.Default.ContainsKey(dialect)
            ? dialect
            : throw new InvalidOperationException($"Unknown SQL dialect \"{dialect}\".");

    public static string Require(string language)
    {
        DriverOf(language);
        return language;
    }

    public static string RenderSql(string dialect, IStatement statement)
    {
        var sql = DriverOf(dialect).StatementRenderer.Render(statement).TrimEnd();
        return sql == "" ? "" : sql.EndsWith(';') || sql.EndsWith('/') ? sql : $"{sql};";
    }

    public static string Q(string dialect, string ident)
    {
        var driver = DriverOf(dialect);
        return $"{driver.QuoteLeft}{ident}{driver.QuoteRight}";
    }

    public static string MapColumnType(string dialect, SpecField field) =>
        DriverOf(dialect).TypeConverter.Convert(field);

    public static string? SqlDefault(string dialect, SpecField field) =>
        field.DefaultValue is null ? null : DriverOf(dialect).DefaultValueConverter.Convert(field.Type, Convert.ToString(field.DefaultValue) ?? "");

    static string? Quote(IDriver driver, string name) =>
        driver.GetType().GetProperty(name, BindingFlags.Public | BindingFlags.Instance)?.GetValue(driver) as string;
}
