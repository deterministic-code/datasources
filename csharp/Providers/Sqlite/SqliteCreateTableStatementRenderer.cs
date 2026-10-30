using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteCreateTableStatementRenderer : CreateTableStatementTextRenderer
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

    protected override string Quote(string name) => SqliteDriver.Quote(name);

    protected override string Type(Column column) => SqliteDriver.NativeType(column);

    protected override string Default(Column column) =>
        column.Default is null
            ? ""
            : column.Default.Name is null
                ? $" DEFAULT {SqliteDriver.NativeDefault(column)}"
                : $" CONSTRAINT {Quote(column.Default.Name)} DEFAULT {SqliteDriver.NativeDefault(column)}";

    protected override string Identity(Column column) => column.Identity ? " PRIMARY KEY AUTOINCREMENT" : "";
}
