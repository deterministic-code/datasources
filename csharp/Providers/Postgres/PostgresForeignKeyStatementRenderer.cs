using Datasources;

namespace Datasources.Postgres;

internal sealed class PostgresForeignKeyStatementRenderer
{
    public string Render(ForeignKeyStatement statement)
    {
        var table = PostgresDriver.Quote(statement.Table);
        var name = PostgresDriver.Quote(statement.Name);
        if (statement.Kind == "drop")
            return $"ALTER TABLE {table} DROP CONSTRAINT {name}";
        var cols = string.Join(", ", statement.Columns.Select(PostgresDriver.Quote));
        var refs = string.Join(", ", statement.ReferencesColumns.Select(PostgresDriver.Quote));
        return $"ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {PostgresDriver.Quote(statement.ReferencesTable)} ({refs})";
    }
}
