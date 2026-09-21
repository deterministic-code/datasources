namespace Datasources;

public sealed record Response(
    Value Result,
    int RowsAffected,
    IReadOnlyList<Parameter>? Parameters = null)
{
    public static Response Of(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null) =>
        new(
            result is Value.Null
                ? parameters?.FirstOrDefault(p => p.Direction == ParameterDirection.ReturnValue)?.Value ?? result
                : result,
            rowsAffected,
            parameters);
}
