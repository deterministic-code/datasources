namespace Datasources.Sqlite;

public sealed record SqliteColumn(string Name, string Type, bool Id = false);

public static class SqliteStatementBuilder
{
    public static string Quote(string id) => $"\"{id.Replace("\"", "\"\"")}\"";

    public static string QuoteQualified(string id) =>
        string.Join(".", id.Split('.').Select(Quote));

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
        new CreateTableStatement(Quote(table), [.. columns.Select(column =>
            new Column(Quote(column.Name), column.Id ? $"{ColumnType(column.Type)} PRIMARY KEY" : ColumnType(column.Type)))]);

    public static IQuery CreateQuery(
        string table,
        IReadOnlyList<string> fields,
        IWhere? where = null,
        IReadOnlyList<Join>? joins = null) =>
        new QueryStatement(
            Quote(table),
            [.. fields.Select(QuoteQualified)],
            where,
            joins is null ? null : [.. joins.Select(join =>
                join with
                {
                    Table = QuoteQualified(join.Table),
                    LeftColumn = QuoteQualified(join.LeftColumn),
                    RightColumn = QuoteQualified(join.RightColumn),
                })]);

    public static IInsertStatement CreateInsert(string table, IReadOnlyList<Parameter> fields) =>
        new InsertStatement(Quote(table), fields);

    public static IStoredProcedureStatement CreateStoredProcedure(
        string name, IReadOnlyList<Parameter>? parameters = null) =>
        new StoredProcedureStatement(Quote(name), parameters ?? []);

    public static IUpdateStatement CreateUpdate(string table, IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        new UpdateStatement(Quote(table), fields, where);

    public static IDeleteStatement CreateDelete(string table, IWhere? where = null) =>
        new DeleteStatement(Quote(table), where);
}
