namespace Datasources;

public sealed class SpecField
{
    public string Type { get; init; } = "";
    public string? Name { get; init; }
    public object? Size { get; init; }
    public object? DefaultValue { get; init; }
    public bool IsOptimisticConcurrency { get; init; }
    public bool UseNativeRowVersion { get; init; }

    public static bool UsesNativeRowVersion(SpecField field) =>
        field.UseNativeRowVersion && field.IsOptimisticConcurrency;

    public static bool EmitsNativeRowVersion(SpecField field, string dialect) =>
        dialect == "sqlserver" && UsesNativeRowVersion(field);

    public static bool IsOccBinaryColumn(SpecField field) =>
        field.Type == "binary" && field.IsOptimisticConcurrency;
}
