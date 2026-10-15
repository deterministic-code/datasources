namespace Datasources.CodeGenerators;

public sealed class SqlMapping
{
    readonly PackCasing casing;
    readonly IReadOnlyDictionary<string, SqlTable> byLogicalName;

    public SqlMapping(PackCasing casing, IReadOnlyList<SqlTable> tables)
    {
        this.casing = casing;
        byLogicalName = tables.ToDictionary(t => t.Name, StringComparer.Ordinal);
    }

    public string TableStem(SqlTable table) => table.Mapping ?? table.Name;

    public string FieldStem(SqlTable table, string fieldName) =>
        SqlSchema.FieldOverlay(table, fieldName)?.Mapping ?? fieldName;

    public string TableName(SqlTable table) => casing.TableName(TableStem(table));

    public string ColumnName(SqlTable table, string fieldName) =>
        casing.ColumnName(FieldStem(table, fieldName));

    public string ConstraintName(SqlTable table, params string[] parts) =>
        casing.ConstraintName(TableStem(table), parts);

    public string TriggerName(SqlTable table) => casing.TriggerName(TableStem(table));

    public SqlTable TableOf(string logicalName) =>
        byLogicalName.TryGetValue(logicalName, out var table)
            ? table
            : throw new InvalidOperationException($"foreign key references unknown table \"{logicalName}\"");

    public (string TableName, string ColumnName) ResolveReference(SpecTypeField field)
    {
        var (logicalTable, logicalCol) = SpecTables.ReferenceTarget(field);
        var target = TableOf(logicalTable);
        return (TableName(target), ColumnName(target, logicalCol));
    }
}
