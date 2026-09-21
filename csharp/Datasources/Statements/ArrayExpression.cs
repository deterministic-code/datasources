namespace Datasources;

public sealed record ArrayExpression(IReadOnlyList<IExpression> Elements) : IExpression;
