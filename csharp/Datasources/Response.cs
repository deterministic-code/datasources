namespace Datasources;

public sealed record Response
{
    public Value Result { get; }
    public int RowsAffected { get; }
    public IReadOnlyList<Parameter>? Parameters { get; }

    public Response(Value result, int rowsAffected, IReadOnlyList<Parameter>? parameters = null)
    {
        Result = result switch
        {
            Value.Null => parameters?.FirstOrDefault(p => p.Direction == ParameterDirection.ReturnValue)?.Value ?? result,
            _ => result
        };
        RowsAffected = rowsAffected;
        Parameters = parameters;
    }
}
