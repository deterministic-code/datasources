namespace Datasources;

public sealed record Subquery(SelectStatement Query) : IExpression;
