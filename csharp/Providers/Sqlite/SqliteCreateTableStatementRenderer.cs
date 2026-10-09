using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteCreateTableStatementRenderer : CreateTableStatementTextRenderer
{
    public override string Render(CreateTableStatement statement)
    {
        var identity = statement.Columns.Where(column => column.Identity).Select(column => column.Name).ToHashSet();
        var constraints = statement.Constraints
            .Where(constraint => constraint is not PrimaryKey pk || !pk.Columns.All(identity.Contains))
            .ToList();
        return base.Render(constraints.Count == statement.Constraints.Count
            ? statement
            : new CreateTableStatement(statement.Table, statement.Columns, constraints));
    }

    protected override string Type(Column column) =>
        SqliteDriver.ColumnTypes.TryGetValue(column.Type, out var sql) ? sql : column.Type;

    protected override string Identity(Column column) => column.Identity ? " PRIMARY KEY AUTOINCREMENT" : "";
}
