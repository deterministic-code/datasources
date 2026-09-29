namespace Datasources;

public sealed record SelectInsertedIdStatement(string Table, string Column) : IStatement
{
    public string GetText() =>
        throw new InvalidOperationException("provider must implement SelectInsertIdStatement");
}
