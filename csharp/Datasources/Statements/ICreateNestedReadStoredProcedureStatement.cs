namespace Datasources;

public interface ICreateNestedReadStoredProcedureStatement : IStatement
{
    string Name { get; }
    NestedSelectStatement Nested { get; }
    IReadOnlyList<NestedWhereArgument> Arguments { get; }
}
