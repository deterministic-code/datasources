namespace Datasources.Builders;

public static class StatementBuilder
{
    public static IQuery CreateQuery(string table, IReadOnlyList<string> fields, IWhere? where = null) =>
        new Query(WithWhere($"SELECT {SelectList(fields)} FROM {table}", where), where);

    public static IInsertStatement CreateInsert(string table, IReadOnlyList<Parameter> fields) =>
        new InsertStatement($"INSERT INTO {table} ({Names(fields)}) VALUES ({Placeholders(fields)})");

    public static IUpdateStatement CreateUpdate(string table, IReadOnlyList<Parameter> fields, IWhere? where = null) =>
        new UpdateStatement(WithWhere($"UPDATE {table} SET {Assignments(fields)}", where), where);

    public static IDeleteStatement CreateDelete(string table, IWhere? where = null) =>
        new DeleteStatement(WithWhere($"DELETE FROM {table}", where), where);

    public static ICreateTableStatement CreateTable(string table, IReadOnlyList<Column> columns) =>
        new CreateTableStatement($"CREATE TABLE {table} ({string.Join(", ", columns.Select(column => $"{column.Name} {column.Type}"))})");

    public static IStoredProcedureStatement CreateStoredProcedure(string name, IReadOnlyList<Parameter>? parameters = null) =>
        new StoredProcedureStatement($"CALL {name}({Placeholders(parameters ?? [])})");

    private static string SelectList(IReadOnlyList<string> fields) =>
        fields.Count == 0 ? "*" : string.Join(", ", fields);

    private static string Names(IReadOnlyList<Parameter> fields) =>
        string.Join(", ", fields.Select(field => field.Name));

    private static string Placeholders(IReadOnlyList<Parameter> fields) =>
        string.Join(", ", fields.Select(field => $"${field.Name}"));

    private static string Assignments(IReadOnlyList<Parameter> fields) =>
        string.Join(", ", fields.Select(field => $"{field.Name} = ${field.Name}"));

    private static string WithWhere(string sql, IWhere? where) =>
        where is null ? sql : $"{sql} WHERE {where.GetText()}";
}
