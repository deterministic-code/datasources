using System.Globalization;
using Backend.Types.View;

namespace Outliers;

public static class Format
{
    public static string TreeLine(CategoryTree node)
    {
        var kids = node.Children.OrderBy(child => child.Name).Select(TreeLine);
        return string.Join(" ", new[] { node.Name }.Concat(kids));
    }

    public static string RegionLine(RegionCategoryTree node)
    {
        var kids = node.Children.OrderBy(child => child.Name).Select(RegionLine);
        return string.Join(" ", new[] { $"{node.RegionId}-{node.CategoryId}/{node.Name}" }.Concat(kids));
    }

    public static string GraphLine(NodeGraph node)
    {
        var edges = node.Edges.Select(edge => edge.Value).OrderBy(value => value).ToArray();
        return edges.Length == 0 ? node.Value : $"{node.Value} {string.Join(",", edges)}";
    }

    public static IEnumerable<string> MapError(string method, string message) =>
        [$"map {method} error {message}"];

    public static IEnumerable<string> MapLines(string method, MappingCell row) =>
        Cells(row).Select(pair => Line(method, pair.Column, pair.Value)).ToArray();

    public static string[] Transcript(
        IEnumerable<CategoryTree> trees,
        IEnumerable<RegionCategoryTree> regions,
        IEnumerable<NodeGraph> graphs,
        IEnumerable<string>? maps = null) =>
        trees.Select(row => $"tree {TreeLine(row)}")
            .Concat(regions.Select(row => $"region {RegionLine(row)}"))
            .Concat(graphs.Select(row => $"graph {GraphLine(row)}"))
            .Concat(maps ?? [])
            .OrderBy(line => line)
            .ToArray();

    static string Line(string method, string column, string value) =>
        value.Length == 0 ? $"map {method} {column}" : $"map {method} {column} {value}";

    static IEnumerable<(string Column, string Value)> Cells(MappingCell row)
    {
        yield return ("id", Text(row.Id));
        yield return ("text_to_string", row.TextToString);
        yield return ("text_to_integer", Text(row.TextToInteger));
        yield return ("text_to_decimal", Text(row.TextToDecimal));
        yield return ("text_to_datetime", Text(row.TextToDatetime));
        yield return ("text_to_binary", Text(row.TextToBinary));
        yield return ("text_to_guid", Text(row.TextToGuid));
        yield return ("int_to_integer", Text(row.IntToInteger));
        yield return ("int_to_float", Text(row.IntToFloat));
        yield return ("int_to_decimal", Text(row.IntToDecimal));
        yield return ("int_to_boolean", Text(row.IntToBoolean));
        yield return ("float_to_integer", Text(row.FloatToInteger));
        yield return ("float_to_float", Text(row.FloatToFloat));
        yield return ("float_to_decimal", Text(row.FloatToDecimal));
        yield return ("decimal_to_integer", Text(row.DecimalToInteger));
        yield return ("decimal_to_float", Text(row.DecimalToFloat));
        yield return ("decimal_to_decimal", Text(row.DecimalToDecimal));
        yield return ("bool_to_boolean", Text(row.BoolToBoolean));
        yield return ("datetime_to_datetime", Text(row.DatetimeToDatetime));
        yield return ("binary_to_binary", Text(row.BinaryToBinary));
        yield return ("binary_to_guid", Text(row.BinaryToGuid));
        yield return ("guid_to_string", row.GuidToString);
        yield return ("guid_to_guid", Text(row.GuidToGuid));
    }

    static string Text(long value) => value.ToString(CultureInfo.InvariantCulture);
    static string Text(double value) => value.ToString("G17", CultureInfo.InvariantCulture);
    static string Text(decimal value) => value.ToString(CultureInfo.InvariantCulture);
    static string Text(bool value) => value ? "true" : "false";
    static string Text(Guid value) => value == Guid.Empty ? "" : value.ToString("D");
    static string Text(byte[] value) => Convert.ToHexString(value).ToLowerInvariant();
    static string Text(DateTime value) =>
        value == default
            ? ""
            : DateTime.SpecifyKind(value, DateTimeKind.Utc)
                .ToString("yyyy-MM-dd'T'HH:mm:ss.fff'Z'", CultureInfo.InvariantCulture);
}
