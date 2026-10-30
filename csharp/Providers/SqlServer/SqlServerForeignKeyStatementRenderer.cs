using Datasources;

namespace Datasources.SqlServer;

internal sealed class SqlServerForeignKeyStatementRenderer
{
    public string Render(ForeignKeyStatement statement)
    {
        var table = SqlServerDriver.Quote(statement.Table);
        var name = SqlServerDriver.Quote(statement.Name);
        if (statement.Kind == "drop")
            return $"ALTER TABLE {table} DROP CONSTRAINT {name}";
        var cols = string.Join(", ", statement.Columns.Select(SqlServerDriver.Quote));
        var refs = string.Join(", ", statement.ReferencesColumns.Select(SqlServerDriver.Quote));
        return $"ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {SqlServerDriver.Quote(statement.ReferencesTable)} ({refs})";
    }
}
