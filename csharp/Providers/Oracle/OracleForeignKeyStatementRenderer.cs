using Datasources;

namespace Datasources.Oracle;

internal sealed class OracleForeignKeyStatementRenderer
{
    public string Render(ForeignKeyStatement statement)
    {
        var table = OracleDriver.Quote(statement.Table);
        var name = OracleDriver.Quote(statement.Name);
        if (statement.Kind == "drop")
            return $"ALTER TABLE {table} DROP CONSTRAINT {name}";
        var cols = string.Join(", ", statement.Columns.Select(OracleDriver.Quote));
        var refs = string.Join(", ", statement.ReferencesColumns.Select(OracleDriver.Quote));
        return $"ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {OracleDriver.Quote(statement.ReferencesTable)} ({refs})";
    }
}
