using Datasources.Builders;

namespace Datasources.Mysql;

public sealed record MysqlColumn(string Name, string Type, bool Id = false);

public static class MysqlStatementBuilder
{
    public static string Quote(string id) => $"`{id.Replace("`", "``")}`";

    public static string ColumnType(string canonical) => canonical switch
    {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new DatasourceException($"Unknown mysql type \"{canonical}\"")
    };

    public static ICreateTableStatement CreateTable(string table, IReadOnlyList<MysqlColumn> columns) =>
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
