using Datasources;

namespace Datasources.Mysql;

internal sealed class MysqlDropTableStatementRenderer : DropTableStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}

internal sealed class MysqlCreateIndexStatementRenderer : CreateIndexStatementTextRenderer
{
    protected override string Quote(string name) => MysqlDriver.Quote(name);
}

internal sealed class MysqlRebuildTableStatementRenderer
{
    public string Render(RebuildTableStatement statement)
    {
        var dest = statement.Create.Table;
        var tmp = $"{dest}__new";
        var old = $"{statement.Table}__old";
        var rebuilt = new CreateTableStatement(tmp, statement.Create.Columns, statement.Create.Constraints);
        var names = string.Join(", ", statement.Columns.Select(column => MysqlDriver.Quote(column.Name)));
        var values = string.Join(", ", statement.Columns.Select(Select));
        var from = MysqlDriver.Quote(statement.Table);
        var to = MysqlDriver.Quote(dest);
        var tmpName = MysqlDriver.Quote(tmp);
        var oldName = MysqlDriver.Quote(old);
        List<string> parts =
        [
            "SET FOREIGN_KEY_CHECKS = 0",
            new MysqlCreateTableStatementRenderer().Render(rebuilt),
            $"INSERT INTO {tmpName} ({names}) SELECT {values} FROM {from}",
        ];
        parts.AddRange(statement.Recreate.Select(item => Recreate(item, tmp)));
        parts.Add($"RENAME TABLE {from} TO {oldName}, {tmpName} TO {to}");
        parts.Add(new MysqlDropTableStatementRenderer().Render(new DropTableStatement(old)));
        parts.Add("SET FOREIGN_KEY_CHECKS = 1");
        return string.Join(";\n", parts.Where(sql => sql != ""));
    }

    static string Select(RebuildColumn column) =>
        column.Source is { } source
            ? column.Cast is { } cast
                ? $"CAST({MysqlDriver.Quote(source)} AS {CastTarget(cast)})"
                : MysqlDriver.Quote(source)
            : column.Literal ?? "NULL";

    static string CastTarget(string type)
    {
        var native = type.ToUpperInvariant();
        return native.Contains("INT") || native.Contains("BOOL") ? "SIGNED"
            : native.Contains("CHAR") || native.Contains("TEXT") || native.Contains("UUID") ? "CHAR"
            : native.Contains("DATE") || native.Contains("TIME") ? "DATETIME"
            : native.Contains("BLOB") || native.Contains("BINARY") ? "BINARY"
            : native.Contains("DEC") || native.Contains("NUM") || native.Contains("FLOAT") || native.Contains("DOUBLE") || native.Contains("REAL") ? "DECIMAL"
            : "CHAR";
    }

    static string Recreate(IStatement statement, string table) => statement switch
    {
        CreateIndexStatement index => new MysqlCreateIndexStatementRenderer().Render(
            new CreateIndexStatement(index.Name, table, index.Columns, index.Unique)),
        CreateTriggerStatement trigger => new MysqlCreateTriggerStatementRenderer().Render(
            new CreateTriggerStatement(table, trigger.Name, trigger.UpdatedColumn, trigger.KeyColumn)),
        _ => throw new InvalidOperationException($"{statement.GetType().Name} cannot be recreated during a table rebuild"),
    };
}
