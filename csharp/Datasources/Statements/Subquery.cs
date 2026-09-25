namespace Datasources;

public sealed record Subquery(QueryStatement Query) : IExpression;
