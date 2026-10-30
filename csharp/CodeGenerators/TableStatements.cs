namespace Datasources.CodeGenerators;

public static class TableStatements
{
    public static CreateTableStatement CreateTable(SqlTable table, SqlMapping mapping)
    {
        var keys = SqlSchema.TableIdentity(table);
        var columns = table.Fields.Select(field => ColumnOf(mapping, table, field)).ToList();
        var uniques = table.Fields
            .Where(field => SqlSchema.FieldOverlay(table, field.Name)?.IsUnique == true || field.Name == "uuid")
            .Select(field => new UniqueConstraint(
                mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "unique_constraint"),
                [mapping.ColumnName(table, field.Name)]))
            .ToList();
        var foreignKeys = table.Fields
            .Where(field => field.References is not null || field.ReferencePair is not null)
            .Select(field => ForeignKeyOf(mapping, table, field))
            .ToList();
        var overlay = SqlSchema.OverlayOf(table);
        IReadOnlyList<ITableConstraint> pk = keys.Count > 1
            ? [new PrimaryKey(mapping.ConstraintName(table, "primary_key"), keys.Select(key => mapping.ColumnName(table, key)).ToList())]
            : keys.Count == 1 && table.Fields.Any(field => SpecTables.IsPkField(field, table.AsType(), overlay))
                ? [new PrimaryKey(mapping.ConstraintName(table, "primary_key"), [mapping.ColumnName(table, keys[0])])]
                : [];
        return new CreateTableStatement(table.TableName, columns, [.. pk, .. uniques, .. foreignKeys]);
    }

    public static CreateIndexStatement CreateIndex(SqlMapping mapping, SqlTable table, DatasourceIndex index) =>
        new(
            mapping.ConstraintName(table, index.Name, "index"),
            table.TableName,
            index.Fields.Select(column => mapping.ColumnName(table, column)).ToList(),
            index.IsUnique);

    public static DropTableStatement DropTable(string table) => new(table);

    public static DropIndexStatement DropIndex(string name, string table) => new(name, table);

    public static DropTriggerStatement DropTrigger(string name, string table) => new(name, table);

    public static Column Column(SqlMapping mapping, SqlTable table, SpecTypeField field) =>
        ColumnOf(mapping, table, field);

    public static UniqueConstraint UniqueOf(SqlMapping mapping, SqlTable table, SpecTypeField field) =>
        new(
            mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "unique_constraint"),
            [mapping.ColumnName(table, field.Name)]);

    public static ForeignKey ForeignKeyOf(SqlMapping mapping, SqlTable table, SpecTypeField field)
    {
        var reference = mapping.ResolveReference(field);
        return new ForeignKey(
            mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "foreign_key"),
            [mapping.ColumnName(table, field.Name)],
            reference.TableName,
            [reference.ColumnName]);
    }

    public static ForeignKeyStatement AddForeignKey(SqlMapping mapping, SqlTable table, SpecTypeField field)
    {
        var fk = ForeignKeyOf(mapping, table, field);
        return new ForeignKeyStatement(table.TableName, fk.Name, fk.Columns, fk.ReferencesTable, fk.ReferencesColumns);
    }

    public static ForeignKeyStatement DropForeignKey(SqlMapping mapping, SqlTable table, SpecTypeField field) =>
        new(table.TableName, ForeignKeyOf(mapping, table, field).Name, [], "", [], "drop");

    public static CreateTriggerStatement CreateTrigger(SqlTable table, SqlMapping mapping, string key) =>
        new(
            table.TableName,
            mapping.TriggerName(table),
            mapping.ColumnName(table, "updated"),
            mapping.ColumnName(table, key));

    public static IdentityInsertStatement IdentityInsert(SqlTable table, SqlMapping mapping, IReadOnlyList<InsertStatement> inserts, bool identity)
    {
        var keys = SqlSchema.TableIdentity(table);
        return new IdentityInsertStatement(
            table.TableName,
            mapping.ColumnName(table, keys.FirstOrDefault() ?? "id"),
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

    static Column ColumnOf(SqlMapping mapping, SqlTable table, SpecTypeField field)
    {
        var converterField = field.ConverterField();
        var generated = SqlSchema.UsesGeneratedIdColumn(table, field);
        var named = mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "default_constraint");
        var fromSpec = converterField.DefaultValue;
        var identity = generated && fromSpec is null;
        var keys = SqlSchema.TableIdentity(table);
        return new Column(
            mapping.ColumnName(table, field.Name),
            converterField.Type,
            fromSpec is null ? null : new ColumnDefault(Convert.ToString(fromSpec) ?? "", named),
            !field.IsNullable || keys.Contains(field.Name) ? new NotNull() : null,
            identity,
            converterField);
    }
}
