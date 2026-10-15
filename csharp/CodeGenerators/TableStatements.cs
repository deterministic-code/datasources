namespace Datasources.CodeGenerators;

public static class TableStatements
{
    public static CreateTableStatement CreateTable(string dialect, SqlTable table, SqlMapping mapping)
    {
        var keys = SqlSchema.TableIdentity(table);
        var columns = table.Fields.Select(field => ColumnOf(dialect, mapping, table, field)).ToList();
        var uniques = table.Fields
            .Where(field => SqlSchema.FieldOverlay(table, field.Name)?.IsUnique == true || field.Name == "uuid")
            .Select(field => new UniqueConstraint(
                SqlDialect.Q(dialect, mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "unique_constraint")),
                [SqlDialect.Q(dialect, mapping.ColumnName(table, field.Name))]))
            .ToList();
        var foreignKeys = table.Fields
            .Where(field => field.References is not null || field.ReferencePair is not null)
            .Select(field =>
            {
                var reference = mapping.ResolveReference(field);
                return new ForeignKey(
                    SqlDialect.Q(dialect, mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "foreign_key")),
                    [SqlDialect.Q(dialect, mapping.ColumnName(table, field.Name))],
                    SqlDialect.Q(dialect, reference.TableName),
                    [SqlDialect.Q(dialect, reference.ColumnName)]);
            })
            .ToList();
        var overlay = SqlSchema.OverlayOf(table);
        IReadOnlyList<ITableConstraint> pk = keys.Count > 1
            ? [new PrimaryKey(SqlDialect.Q(dialect, mapping.ConstraintName(table, "primary_key")), keys.Select(key => SqlDialect.Q(dialect, mapping.ColumnName(table, key))).ToList())]
            : keys.Count == 1 && table.Fields.Any(field => SpecTables.IsPkField(field, table.AsType(), overlay))
                ? [new PrimaryKey(SqlDialect.Q(dialect, mapping.ConstraintName(table, "primary_key")), [SqlDialect.Q(dialect, mapping.ColumnName(table, keys[0]))])]
                : [];
        return new CreateTableStatement(SqlDialect.Q(dialect, table.TableName), columns, [.. pk, .. uniques, .. foreignKeys]);
    }

    public static CreateIndexStatement CreateIndex(string dialect, SqlMapping mapping, SqlTable table, DatasourceIndex index) =>
        new(
            SqlDialect.Q(dialect, mapping.ConstraintName(table, index.Name, "index")),
            SqlDialect.Q(dialect, table.TableName),
            index.Fields.Select(column => SqlDialect.Q(dialect, mapping.ColumnName(table, column))).ToList(),
            index.IsUnique);

    public static DropTableStatement DropTable(string dialect, string table) =>
        new(SqlDialect.Q(dialect, table));

    public static CreateTriggerStatement CreateTrigger(string dialect, SqlTable table, SqlMapping mapping, string key) =>
        new(
            SqlDialect.Q(dialect, table.TableName),
            SqlDialect.Q(dialect, mapping.TriggerName(table)),
            SqlDialect.Q(dialect, mapping.ColumnName(table, "updated")),
            SqlDialect.Q(dialect, mapping.ColumnName(table, key)),
            SqlDialect.DriverOf(dialect).DefaultValueConverter.Convert("datetime", "UtcNow") ?? "");

    public static InsertStatement SeedInsert(SqlTable table, SqlMapping mapping, IReadOnlyList<string> columns, IReadOnlyList<string> values, string dialect) =>
        new(
            SqlDialect.Q(dialect, table.TableName),
            columns.Select((column, i) => new Parameter(
                SqlDialect.Q(dialect, mapping.ColumnName(table, column)),
                SqlValue(values.ElementAtOrDefault(i) ?? "NULL"))).ToList());

    public static IdentityInsertStatement IdentityInsert(string dialect, SqlTable table, SqlMapping mapping, IReadOnlyList<InsertStatement> inserts, bool identity)
    {
        var keys = SqlSchema.TableIdentity(table);
        return new IdentityInsertStatement(
            table.TableName,
            mapping.ColumnName(table, keys.FirstOrDefault() ?? "id"),
            inserts,
            identity);
    }

    public static CreateRoutineStatement CreateRoutine(
        string name,
        RoutineKind kind,
        string table,
        IReadOnlyList<string> columns,
        IReadOnlyList<RoutineParameter> parameters,
        string key,
        string keyType,
        IReadOnlyList<string> writable,
        string? byField = null,
        bool generatedKey = false,
        string? auditUpdated = null) =>
        new(name, kind, table, columns, parameters, key, keyType, writable, byField, generatedKey, auditUpdated);

    public static DropRoutineStatement DropRoutine(string name) => new(name);

    static Column ColumnOf(string dialect, SqlMapping mapping, SqlTable table, SpecTypeField field)
    {
        var converterField = field.ConverterField();
        var generated = SqlSchema.UsesGeneratedIdColumn(table, field);
        var identity = generated && field.Type is not "uuid" and not "string";
        var named = mapping.ConstraintName(table, mapping.FieldStem(table, field.Name), "default_constraint");
        var utcNow = SqlDialect.DriverOf(dialect).DefaultValueConverter.Convert("datetime", "UtcNow");
        var newId = SqlDialect.DriverOf(dialect).DefaultValueConverter.Convert("uuid", "NewId");
        var fromSpec = SqlDialect.SqlDefault(dialect, converterField);
        var defaultExpr = fromSpec
            ?? (field.Name is "created" or "updated" ? utcNow : null)
            ?? (field.Name == "uuid" || generated && field.Type == "uuid" ? newId : null);
        var keys = SqlSchema.TableIdentity(table);
        return new Column(
            SqlDialect.Q(dialect, mapping.ColumnName(table, field.Name)),
            SqlDialect.MapColumnType(dialect, converterField),
            defaultExpr is null ? null : new ColumnDefault(defaultExpr, SqlDialect.Q(dialect, named)),
            !field.IsNullable || keys.Contains(field.Name) ? new NotNull() : null,
            identity);
    }

    static Value SqlValue(string sql) => sql == "NULL" ? Value.Null.Instance : new Value.Text(sql);
}
