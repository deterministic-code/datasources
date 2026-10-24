namespace Datasources;

public sealed record CreateNestedReadStoredProcedureStatement(
    string Name,
    NestedSelectStatement Nested,
    IReadOnlyList<NestedWhereArgument> Arguments) : ICreateNestedReadStoredProcedureStatement
{
    public SelectStatement Select => WithArguments(Nested.Select, Arguments);

    public string Accept(IStatementVisitor visitor) => visitor.Visit(this);

    public static SelectStatement WithArguments(SelectStatement select, IReadOnlyList<NestedWhereArgument> arguments)
    {
        var where = arguments.Aggregate(select.Where, (clause, argument) =>
        {
            var equals = new Operator.Equals(new Identifier(argument.Path), new Identifier(argument.Parameter.Name));
            return clause is Where existing
                ? new Where(new Operator.And(existing.Condition, equals))
                : new Where(equals);
        });
        return new SelectStatement(new()
        {
            Distinct = select.Distinct,
            Table = select.Table ?? "",
            Fields = select.Fields,
            Aliases = select.Aliases,
            Joins = select.Joins,
            Where = where,
            GroupBy = select.GroupBy,
            Having = select.Having,
            OrderBy = select.OrderBy,
            Limit = select.Limit,
            Offset = select.Offset,
        });
    }
}
