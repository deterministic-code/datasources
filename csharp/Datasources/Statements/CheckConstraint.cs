namespace Datasources;

public sealed record CheckConstraint(string Name, string Expression) : ITableConstraint;
