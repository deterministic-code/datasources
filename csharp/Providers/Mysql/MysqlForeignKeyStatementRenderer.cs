using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlForeignKeyStatementRenderer
{
    public string Render(ForeignKeyStatement statement)
    {
        var table = MysqlDriver.Quote(statement.Table);
        var name = MysqlDriver.Quote(statement.Name);
        if (statement.Kind == "drop")
            return $"ALTER TABLE {table} DROP FOREIGN KEY {name}";
        var cols = string.Join(", ", statement.Columns.Select(MysqlDriver.Quote));
        var refs = string.Join(", ", statement.ReferencesColumns.Select(MysqlDriver.Quote));
        return $"ALTER TABLE {table} ADD CONSTRAINT {name} FOREIGN KEY ({cols}) REFERENCES {MysqlDriver.Quote(statement.ReferencesTable)} ({refs})";
    }
}
