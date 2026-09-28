namespace Datasources;

public sealed record InsertedIdStatement(string Table, string Column) : IStatement
{
    public string GetText() => $"{Table}.{Column}";
}
