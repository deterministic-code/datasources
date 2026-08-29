using Datasources.Builders;

namespace Datasources.Sqlite;

public sealed record SqliteColumn(string Name, string Type, bool Id = false);

public static class SqliteStatementBuilder
{
    public static string Quote(string id) => $"\"{id.Replace("\"", "\"\"")}\"";

    public static string ColumnType(string canonical) => canonical switch
    {
        "int" => "INTEGER",
        "uuid" => "TEXT",
        "string" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown sqlite type \"{canonical}\"")
    };

    public static ICreateTableStatement CreateTable(string table, IReadOnlyList<SqliteColumn> columns) =>
        StatementBuilder.CreateTable(Quote(table), [.. columns.Select(column =>
            new Column(Quote(column.Name), column.Id ? $"{ColumnType(column.Type)} PRIMARY KEY" : ColumnType(column.Type)))]);

    public static IQuery CreateQuery(string table, IReadOnlyList<string> fields, IWhere? where = null) =>
        StatementBuilder.CreateQuery(Quote(table), [.. fields.Select(Quote)], where);

    public static IInsertStatement CreateInsert(string table, IReadOnlyList<Parameter> fields) =>
        StatementBuilder.CreateInsert(Quote(table), fields);

    public static IUpdateStatement CreateUpdate(string table, IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        StatementBuilder.CreateUpdate(Quote(table), fields, where);

    public static IDeleteStatement CreateDelete(string table, IWhere? where = null) =>
        StatementBuilder.CreateDelete(Quote(table), where);
}
