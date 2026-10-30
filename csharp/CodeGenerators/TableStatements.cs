namespace Datasources.CodeGenerators;

public static class TableStatements
{
    public static CreateTableStatement CreateTable(
        SqlTable table,
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables)
    {
        var keys = SqlSchema.TableIdentity(table);
        var columns = table.Fields.Select(field => ColumnOf(naming, table, field)).ToList();
        var stem = SqlSchema.TableStem(table);
        var uniques = table.Fields
            .Where(field => SqlSchema.FieldOverlay(table, field.Name)?.IsUnique == true || field.Name == "uuid")
            .Select(field => new UniqueConstraint(
                UniqueConstraint.NameOf(stem, naming, SqlSchema.FieldStem(table, field.Name), "unique_constraint"),
                [CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, field.Name), naming)]))
            .ToList();
        var foreignKeys = table.Fields
            .Where(field => field.References is not null || field.ReferencePair is not null)
            .Select(field => ForeignKeyOf(naming, tables, table, field))
            .ToList();
        var overlay = SqlSchema.OverlayOf(table);
        IReadOnlyList<ITableConstraint> pk = keys.Count > 1
            ? [new PrimaryKey(PrimaryKey.NameOf(stem, naming, "primary_key"), keys.Select(key => CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, key), naming)).ToList())]
            : keys.Count == 1 && table.Fields.Any(field => SpecTables.IsPkField(field, table.AsType(), overlay))
                ? [new PrimaryKey(PrimaryKey.NameOf(stem, naming, "primary_key"), [CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, keys[0]), naming)])]
                : [];
        return new CreateTableStatement(table.TableName, columns, [.. pk, .. uniques, .. foreignKeys]);
    }

    public static CreateIndexStatement CreateIndex(StatementNaming naming, SqlTable table, DatasourceIndex index) =>
        new(
            CreateIndexStatement.NameOf(SqlSchema.TableStem(table), naming, index.Name, "index"),
            table.TableName,
            index.Fields.Select(column => CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, column), naming)).ToList(),
            index.IsUnique);

    public static DropTableStatement DropTable(string table) => new(table);

    public static DropIndexStatement DropIndex(string name, string table) => new(name, table);

    public static DropTriggerStatement DropTrigger(string name, string table) => new(name, table);

    public static Column Column(StatementNaming naming, SqlTable table, SpecTypeField field) =>
        ColumnOf(naming, table, field);

    public static UniqueConstraint UniqueOf(StatementNaming naming, SqlTable table, SpecTypeField field) =>
        new(
            UniqueConstraint.NameOf(SqlSchema.TableStem(table), naming, SqlSchema.FieldStem(table, field.Name), "unique_constraint"),
            [CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, field.Name), naming)]);

    public static ForeignKey ForeignKeyOf(
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables,
        SqlTable table,
        SpecTypeField field)
    {
        var reference = SqlSchema.ResolveReference(tables, naming, field);
        return new ForeignKey(
            ForeignKey.NameOf(SqlSchema.TableStem(table), naming, SqlSchema.FieldStem(table, field.Name), "foreign_key"),
            [CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, field.Name), naming)],
            reference.TableName,
            [reference.ColumnName]);
    }

    public static ForeignKeyStatement AddForeignKey(
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables,
        SqlTable table,
        SpecTypeField field)
    {
        var fk = ForeignKeyOf(naming, tables, table, field);
        return new ForeignKeyStatement(table.TableName, fk.Name, fk.Columns, fk.ReferencesTable, fk.ReferencesColumns);
    }

    public static ForeignKeyStatement DropForeignKey(
        StatementNaming naming,
        IReadOnlyList<SqlTable> tables,
        SqlTable table,
        SpecTypeField field) =>
        new(table.TableName, ForeignKeyOf(naming, tables, table, field).Name, [], "", [], "drop");

    public static CreateTriggerStatement CreateTrigger(SqlTable table, StatementNaming naming, string key) =>
        new(
            table.TableName,
            CreateTriggerStatement.NameOf(SqlSchema.TableStem(table), naming),
            CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, "updated"), naming),
            CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, key), naming));

    public static IdentityInsertStatement IdentityInsert(SqlTable table, StatementNaming naming, IReadOnlyList<InsertStatement> inserts, bool identity)
    {
        var keys = SqlSchema.TableIdentity(table);
        return new IdentityInsertStatement(
            table.TableName,
            CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, keys.FirstOrDefault() ?? "id"), naming),
            inserts,
            identity);
    }

    public static DropRoutineStatement DropRoutine(string name) => new(name);

    public static CreateCreateStoredProcedureStatement CreateCreateStoredProcedure(
        string name, string table, IReadOnlyList<string> columns, IReadOnlyList<RoutineParameter> parameters,
        string key, string keyType, IReadOnlyList<string> writable, bool generatedKey = false, string? auditUpdated = null) =>
        new(name, table, columns, parameters, key, keyType, writable, generatedKey, auditUpdated);

    public static CreateReadStoredProcedureStatement CreateReadStoredProcedure(
        string name, string table, IReadOnlyList<string> columns, IReadOnlyList<RoutineParameter> parameters,
        string key, string keyType, IReadOnlyList<string> writable, IReadOnlyList<string>? byProperties = null,
        bool generatedKey = false, string? auditUpdated = null) =>
        new(name, table, columns, parameters, key, keyType, writable, byProperties ?? [], generatedKey, auditUpdated);

    public static CreateUpdateStoredProcedureStatement CreateUpdateStoredProcedure(
        string name, string table, IReadOnlyList<string> columns, IReadOnlyList<RoutineParameter> parameters,
        string key, string keyType, IReadOnlyList<string> writable, IReadOnlyList<string>? byProperties = null,
        bool generatedKey = false, string? auditUpdated = null) =>
        new(name, table, columns, parameters, key, keyType, writable, byProperties ?? [], generatedKey, auditUpdated);

    public static CreateDeleteStoredProcedureStatement CreateDeleteStoredProcedure(
        string name, string table, IReadOnlyList<string> columns, IReadOnlyList<RoutineParameter> parameters,
        string key, string keyType, IReadOnlyList<string> writable, IReadOnlyList<string>? byProperties = null,
        bool generatedKey = false, string? auditUpdated = null) =>
        new(name, table, columns, parameters, key, keyType, writable, byProperties ?? [], generatedKey, auditUpdated);

    public static CreateBulkCreateStoredProcedureStatement CreateBulkCreateStoredProcedure(
        string name, string table, IReadOnlyList<RoutineParameter> columns, RoutineParameter payload, string key, string keyType) =>
        new(name, table, columns, payload, key, keyType);

    public static CreateBulkUpdateStoredProcedureStatement CreateBulkUpdateStoredProcedure(
        string name, string table, IReadOnlyList<RoutineParameter> columns, RoutineParameter payload, string key, string keyType) =>
        new(name, table, columns, payload, key, keyType);

    public static CreateBulkDeleteStoredProcedureStatement CreateBulkDeleteStoredProcedure(
        string name, string table, IReadOnlyList<RoutineParameter> columns, RoutineParameter payload, string key, string keyType) =>
        new(name, table, columns, payload, key, keyType);

    static Column ColumnOf(StatementNaming naming, SqlTable table, SpecTypeField field)
    {
        var converterField = field.ConverterField();
        var generated = SqlSchema.UsesGeneratedIdColumn(table, field);
        var named = UniqueConstraint.NameOf(
            SqlSchema.TableStem(table), naming, SqlSchema.FieldStem(table, field.Name), "default_constraint");
        var fromSpec = converterField.DefaultValue;
        var identity = generated && fromSpec is null;
        var keys = SqlSchema.TableIdentity(table);
        return new Column(
            CreateTableStatement.ColumnName(SqlSchema.FieldStem(table, field.Name), naming),
            converterField.Type,
            fromSpec is null ? null : new ColumnDefault(Convert.ToString(fromSpec) ?? "", named),
            !field.IsNullable || keys.Contains(field.Name) ? new NotNull() : null,
            identity,
            converterField);
    }
}
